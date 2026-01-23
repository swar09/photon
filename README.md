# photon-db

just a really fast vector db written in rust.
it creates bindings for python so you can use it there too.

## usage

install it (once published) or build with `maturin develop`.

```python
import photon_db

# max_elements, dim, m, ef_construction
db = photon_db.PyHNSW(10000, 128, 16, 64)

# insert some data
# vec, m, m_max, ef_construction, m_l
db.insert([0.1] * 128, 16, 16, 64, 0.5)

# find nearest neighbors
# query, k, ef_search
results = db.search([0.1] * 128, 5, 100)
print(results)
```

thats it.
