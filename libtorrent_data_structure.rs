// Data upload submission and distribution using libtorrent model
// Must research ways to implement this distribution model in http retreivable format for NIP-96 file servers

use std::collections::{HashMap, HashSet};
use std::fs::{self, File as StdFile};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use lz4::block::{compress, decompress};
use reed_solomon_erasure::galois_8::ReedSolomon; // For erasure coding
use sha2::{Digest, Sha256}; // For content-addressable storage
use bsdiff::Bsdiff; // For delta encoding
use rand::seq::SliceRandom;

// Represents a block of data to be distributed
#[derive(Clone)]
struct DataBlock {
    index: usize,
    content: Vec<u8>, // Compressed and encoded content
}

// Metadata for file piece
#[derive(Clone)]
struct PieceMetadata {
    index: usize,
    hash: Vec<u8>, // Content-addressable storage: hashed value of the data piece
    is_rare: bool, // Rarest-first logic
}

// Stores a file split into pieces
struct File {
    pieces: Vec<PieceMetadata>,
    peers: HashMap<String, HashSet<usize>>, // Which peers have which pieces
}

impl File {
    fn new(pieces: Vec<PieceMetadata>) -> Self {
        File {
            pieces,
            peers: HashMap::new(),
        }
    }

    // Track which peer has which pieces
    fn update_peer(&mut self, peer_id: &str, piece_index: usize) {
        self.peers
            .entry(peer_id.to_string())
            .or_insert_with(HashSet::new)
            .insert(piece_index);
    }

    // Get the rarest piece that this peer doesn't have yet
    fn rarest_first_piece(&self, peer_id: &str) -> Option<&PieceMetadata> {
        let peer_pieces = self.peers.get(peer_id);
        self.pieces
            .iter()
            .filter(|piece| match peer_pieces {
                Some(set) => !set.contains(&piece.index),
                None => true,
            })
            .min_by_key(|piece| {
                // Rare = fewest peers have this piece
                self.peers
                    .values()
                    .filter(|peer_set| peer_set.contains(&piece.index))
                    .count()
            })
    }
}

// Compress the data block using lz4
fn compress_block(data: &[u8]) -> Vec<u8> {
    compress(data, None, false).expect("Failed to compress data")
}

// Decompress the data block
fn decompress_block(data: &[u8]) -> Vec<u8> {
    decompress(data, None).expect("Failed to decompress data")
}

// Content-addressable storage: get the hash of a piece of data
fn hash_piece(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

// Apply delta encoding between two data versions
fn delta_encode(old_data: &[u8], new_data: &[u8]) -> Vec<u8> {
    Bsdiff::diff(old_data, new_data).expect("Failed to create delta")
}

// Erasure coding: generate parity blocks
fn erasure_encode(blocks: Vec<Vec<u8>>, parity_count: usize) -> Vec<Vec<u8>> {
    let encoder = ReedSolomon::new(blocks.len(), parity_count).expect("Failed to create encoder");
    let mut shards = blocks.clone();
    encoder.encode(&mut shards).expect("Failed to encode");
    shards
}

// Choking algorithm: prioritize the fastest peers
fn choke_peers(peers: &mut HashMap<String, usize>, choke_limit: usize) -> Vec<String> {
    // Sort peers by the number of pieces they have, limit to `choke_limit`
    let mut peer_list: Vec<_> = peers.iter().collect();
    peer_list.sort_by_key(|(_, pieces)| *pieces);
    peer_list
        .into_iter()
        .take(choke_limit)
        .map(|(peer_id, _)| peer_id.clone())
        .collect()
}

// Full file SHA-256 hashing
fn hash_full_file(file_data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(file_data);
    hasher.finalize().to_vec()
}

// Check the tracker for matching files (simplified; would be done over a network)
fn check_tracker_for_file(hash: &[u8]) -> Option<PathBuf> {
    // Simulate a tracker query. In a real application, this would be a network call to a tracker.
    let tracker_db = vec![
        (b"existing_file_hash_1".to_vec(), PathBuf::from("/path/to/existing/file1")),
        (b"existing_file_hash_2".to_vec(), PathBuf::from("/path/to/existing/file2")),
    ];

    for (stored_hash, path) in tracker_db {
        if stored_hash == hash {
            return Some(path);
        }
    }
    None
}

// Create a symlink to the existing file if it exists on the tracker
fn create_symlink_if_exists(full_file_hash: &[u8], destination: &Path) -> bool {
    if let Some(existing_file_path) = check_tracker_for_file(full_file_hash) {
        // Symlink to the existing file
        if let Err(e) = std::os::unix::fs::symlink(&existing_file_path, destination) {
            eprintln!("Failed to create symlink: {}", e);
        } else {
            println!("Symlinked to existing file at {:?}", existing_file_path);
            return true;
        }
    }
    false
}

// Example function to store and distribute a file
fn store_and_distribute(
    file_data: &[u8],
    peers: &mut HashMap<String, usize>,
    choke_limit: usize,
    pseudo_folder: &Path, // Grouping files in a pseudo-folder
) -> File {
    let mut file = File::new(vec![]);

    // Compute the full file hash
    let full_file_hash = hash_full_file(file_data);

    // Check if the file already exists in the tracker and symlink it if found
    let symlink_created = create_symlink_if_exists(&full_file_hash, pseudo_folder);

    if symlink_created {
        println!("File already exists. Symlink created. No need to store again.");
        return file; // No need to store and distribute the file again
    }

    // Split file into pieces (example: fixed-size chunks)
    let piece_size = 256 * 1024; // 256 KB
    let mut pieces = vec![];

    for (index, chunk) in file_data.chunks(piece_size).enumerate() {
        let compressed_chunk = compress_block(chunk);
        let hash = hash_piece(&compressed_chunk);

        let metadata = PieceMetadata {
            index,
            hash,
            is_rare: false, // Mark all pieces as non-rare initially
        };

        pieces.push(metadata.clone());
        file.pieces.push(metadata);
    }

    // Update rarest pieces
    for piece in &mut file.pieces {
        let peers_with_piece = peers.values().filter(|&&p| p == piece.index).count();
        piece.is_rare = peers_with_piece < 2; // Mark piece rare if fewer than 2 peers have it
    }

    // Choke slow peers
    let active_peers = choke_peers(peers, choke_limit);
    println!("Active peers after choking: {:?}", active_peers);

    // Distribute rarest first
    for peer in active_peers {
        if let Some(piece) = file.rarest_first_piece(&peer) {
            println!("Assigning rarest piece {} to peer {}", piece.index, peer);
        }
    }

    file
}

fn main() {
    // Example file data
    let file_data = b"Example data that needs to be stored and distributed in pieces.";

    // Example peer map (peer_id -> number of pieces they have)
    let mut peers: HashMap<String, usize> = HashMap::new();
    peers.insert("peer1".to_string(), 1);
    peers.insert("peer2".to_string(), 2);

    // Path to pseudo-folder where files will be grouped
    let pseudo_folder = Path::new("/path/to/pseudo_folder");

    // Store and distribute the file
    let file = store_and_distribute(file_data, &mut peers, 1, pseudo_folder);

    // Print file metadata
    println!("File metadata: {:?}", file.pieces);
}
