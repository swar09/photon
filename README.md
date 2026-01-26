# Photon DB: fast vector db in rust

hey so this is Photon DB, its a high performance vector database i built using Rust but with Python bindings. It uses HNSW (Hierarchical Navigable Small World) graphs which makes the nearest neighbor search really fast.

## Features

- **Fast af**: implemented in rust so its optimized for speed
- **Easy Python API**: simple interface to just plug and play
- **Persistence**: saves/loads indexes to disk instantly using zero-copy serialization (`rkyv`)
- **Customizable**: you can fine tune parameters like `M` and `ef_construction` depending on what you need

## Installation

you just need rust toolchain and python 3.7+ installed.

```bash
# 1. make a venv
python3 -m venv venv
source venv/bin/activate

# 2. install dependencies
pip install maturin numpy sentence-transformers tqdm torch --index-url https://download.pytorch.org/whl/cpu

# 3. build and install
maturin develop --release
```

## API Documentation

The main class is `PyHNSW`, here's how to use it.

### Class: `photon_db.PyHNSW`

#### `__init__(max_elements, dim, m, ef_construction)`

initializes the index.

*   `max_elements`: estimate of how many vectors you'll have (index can grow so just a rough number is fine)
*   `dim`: dimensionality of your vectors (e.g. 384 for MiniLM, 1536 for OpenAI)
*   `m`: max outgoing connections per node.
    *   *tip*: 16-64 is usually good. higher = better recall but bigger index size
*   `ef_construction`: candidate list size during build.
    *   *tip*: keep it between 100-500. higher means better graph quality but takes longer to build

#### `insert(vec, m, m_max, ef_construction, m_l)`

inserts a single vector.

*   `vec`: the vector embedding (list of floats)
*   `m`: max connections for this insert (usually same as init)
*   `m_max`: max allowed connections per layer (usually `m * 2`)
*   `ef_construction`: depth for this insert (same as init)
*   `m_l`: level generation factor (default `1.0`)
*   **returns**: the internal doc ID (int)

#### `search(query, k, ef_search)`

does the actual ANN search.

*   `query`: your query vector
*   `k`: how many neighbors you want back
*   `ef_search`: search depth.
    *   *tip*: set this to `k` or `k * 10`. higher value = more accurate but slower latency
*   **returns**: list of results sorted by distance `[(distance, doc_id), ...]`

#### `brute_force_search(query, k)`

does an exact search checking every single vector. mostly just for testing recall/accuracy.

#### `save(path)`

saves the whole graph to disk.

#### `load(path)`

static method to load a saved index.
```python
db = photon_db.PyHNSW.load("my_index.pho")
```

## Benchmarks

**Latest Benchmark Output (SIFT10k)**
```text
    ╔══════════════════════════════════════════════════════════════╗
    ║             Dataset: SIFT10k (128d) | Mode: Strict           ║
    ╚══════════════════════════════════════════════════════════════╝
    
[1/4] Loading SIFT10k Dataset...
      Loaded 10000 base vectors
      Loaded 100 query vectors

[2/4] Building HNSW Index...
  [00:00:03] [████████████████████████████████████████] 10000/10000 (0s)                                                                                                                                                               Index Time: 3.85s (2598 items/sec)

[3/4] Running Queries (k=1)...

[4/4] Verifying against Ground Truth...

════════════════ FINAL RESULTS ════════════════
Engine:              Photon-DB
Dataset:             10000 vectors
Avg Latency:         53 µs
Throughput:          18 items/s
Recall@1:            97%
═══════════════════════════════════════════════
```
