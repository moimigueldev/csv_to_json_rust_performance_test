# Performance Comparison of CSV to JSON Models

### Best Models Rundown

1. **No Vector Memory Solution (Single Threaded)**
   - **Performance**: This model consistently outperformed all others, completing the 2 million record conversion in approximately **33 seconds**.
   - **Reason for Performance**: It avoids holding large data structures in memory by immediately writing each processed record to the output file. This single-threaded approach reduces both memory usage and processing overhead, making it particularly efficient for large datasets.
   - **Key Characteristics**:
     - Memory-efficient: Minimizes RAM usage by avoiding large vector storage.
     - Low Overhead: No parallelization overhead.
   - **Summary**: Best choice for environments with limited memory where predictable, low memory usage is required.

2. **Rayon with Crossbeam Batches (Parallelized)**
   - **Performance**: With careful batching and a bounded channel size, this solution performed well, completing the 2 million record conversion in approximately **49-53 seconds**.
   - **Reason for Performance**: This model leverages Rayon for parallel processing and Crossbeam for bounded channels to limit memory usage. It writes data in smaller chunks, which helps manage resources effectively while utilizing parallel processing.
   - **Key Characteristics**:
     - Parallelized: Uses multiple threads, improving speed over fully sequential operations.
     - Controlled Memory Usage: Bounded channels prevent excessive memory consumption.
   - **Summary**: Best for systems that can benefit from parallel processing but need to control memory usage to avoid overwhelming available resources.

---

### Detailed Performance Summary for Each Model

#### System Specifications
- **Logical CPU Cores**: 2
- **Physical CPU Cores**: 1
- **Total Memory**: 12.67 GB
- **Available Memory**: 11.37 GB
- **Disk Space**: 107.72 GB total, 74.23 GB free

---

### 1. **No Vector Memory Solution**

#### Description
- A single-threaded model that reads each CSV record, processes it, and writes it directly to the output JSON file without holding large batches in memory.

#### Results
- **200k records**:
  - **Real**: 2.948s
  - **User**: 2.258s
  - **Sys**: 0.427s
- **1 million records**:
  - **Real**: 15.234s
  - **User**: 12.206s
  - **Sys**: 2.010s
- **2 million records**:
  - **Real**: 33.026s
  - **User**: 25.865s
  - **Sys**: 4.573s

#### Summary
This model’s direct write approach yielded the fastest times, especially for larger datasets, as it minimized memory usage and eliminated parallelization overhead.

---

### 2. **No Vector Memory Solution (No Nested Loops)**

#### Description
- Similar to the no vector memory solution, but optimized to run without nested loops, aiming to reduce processing overhead in data transformation.

#### Results
- **200k records**:
  - **Real**: 3.569s
  - **User**: 2.861s
  - **Sys**: 0.403s
- **1 million records**:
  - **Real**: 19.423s
  - **User**: 15.321s
  - **Sys**: 2.154s

#### Summary
This solution showed slightly higher processing times than the original no vector memory solution, especially as dataset sizes increased, suggesting nested loops were not a significant performance bottleneck in this context.

---

### 3. **Parallelization with Mutex**

#### Description
- Utilizes multiple threads with a Mutex for synchronization, allowing data to be processed in parallel but requiring synchronization on write access.

#### Results
- **200k records, 1 thread**: 4.854s
- **200k records, 4 threads**: 4.850s
- **200k records, 6 threads**: 3.019s
- **1 million records, 1 thread**: 34.775s
- **1 million records, 6 threads**: 36.062s
- **1 million records, 8 threads**: 31.712s

#### Summary
This model showed diminishing returns with more threads due to the Mutex synchronization overhead, particularly with larger datasets. It performed slower than the single-threaded no vector memory solution.

---

### 4. **Parallelization with Channels**

#### Description
- A multi-threaded model using channels to send processed data to a writer thread, aiming to avoid direct Mutex locking on writes.

#### Results
- **200k records, 1 thread**: 4.812s
- **200k records, 8 threads**: 2.965s
- **1 million records, 1 thread**: 24.713s
- **1 million records, 6 threads**: 25.715s

#### Summary
Channels improved parallelization but could not match the efficiency of the single-threaded no vector memory solution, as the communication overhead between threads added processing time.

---

### 5. **Rayon Batching**

#### Description
- Uses Rayon’s parallel iterators to process records in batches, aiming to reduce memory footprint while benefiting from parallel processing.

#### Results
- **200k records, batch size 100**: 2.789s
- **1 million records, batch size 500**: 25.853s
- **1 million records, batch size 1000**: 35.495s

#### Summary
Rayon batching was more efficient than unbatched parallel processing but still did not outperform the no vector memory solution. Larger batch sizes led to increased memory use and slower performance.

---

### 6. **Rayon Crossbeam with Batches**

#### Description
- Combines Rayon’s parallel processing with Crossbeam’s bounded channels to limit memory usage, writing batches to the output file as they complete.

#### Results
- **200k records, batch size 100, bound 5**: 2.676s
- **1 million records, batch size 100, bound 5**: 18.630s
- **2 million records, batch size 1000, bound 20**: 49.903s

#### Summary
Rayon Crossbeam provided strong performance with smaller batch sizes and bounds, but the single-threaded no vector memory solution remained the most efficient. This model could be preferable in memory-constrained environments due to its bounded memory usage.

---

### Conclusion

The **No Vector Memory Solution** is the most efficient for CSV-to-JSON conversion on large datasets, with minimal memory usage and the fastest runtime. **Rayon Crossbeam with Batches** is the next best alternative when parallel processing is desired but needs careful tuning to balance memory and performance. Multithreading with Mutex or Channels offers no significant advantage in this context due to synchronization and communication overhead.



```python
import psutil

# CPU Information
print(f"CPU cores (logical): {psutil.cpu_count(logical=True)}")
print(f"CPU cores (physical): {psutil.cpu_count(logical=False)}")

# Memory (RAM) Information
mem = psutil.virtual_memory()
print(f"Total memory: {mem.total / (1024 ** 3):.2f} GB")
print(f"Available memory: {mem.available / (1024 ** 3):.2f} GB")
print(f"Used memory: {mem.used / (1024 ** 3):.2f} GB")
print(f"Memory usage: {mem.percent}%")

# Disk Information
disk = psutil.disk_usage('/')
print(f"Total disk space: {disk.total / (1024 ** 3):.2f} GB")
print(f"Used disk space: {disk.used / (1024 ** 3):.2f} GB")
print(f"Free disk space: {disk.free / (1024 ** 3):.2f} GB")
print(f"Disk usage: {disk.percent}%")

```

    CPU cores (logical): 2
    CPU cores (physical): 1
    Total memory: 12.67 GB
    Available memory: 11.37 GB
    Used memory: 0.99 GB
    Memory usage: 10.3%
    Total disk space: 107.72 GB
    Used disk space: 33.47 GB
    Free disk space: 74.23 GB
    Disk usage: 31.1%



```python
!chmod +x ./rust_csv_to_json
```

# no_vec_mem



```python
# Running the smaller 200k line file
!time ./rust_csv_to_json ./chicago_crimes_2024.csv

```

    
    real	0m2.948s
    user	0m2.258s
    sys	0m0.427s



```python
# Running the smaller 1million line file
!time ./rust_csv_to_json ./chicago_crimes_2020_to_2024.csv

```

    
    real	0m15.234s
    user	0m12.206s
    sys	0m2.010s


# no_vec_mem_no_nested_loops

### Updated the code to run on only one loop instead of nested loops



```python
# 200K file
!time ./rust_csv_to_json ./chicago_crimes_2024.csv
```

    
    real	0m3.569s
    user	0m2.861s
    sys	0m0.403s



```python
# 1M file
!time ./rust_csv_to_json ./chicago_crimes_2020_to_2024.csv
```

    
    real	0m19.423s
    user	0m15.321s
    sys	0m2.154s


# Parallelization Mutex


```python
# 200k Lines -> 1 thread
!time ./rust_csv_to_json ./chicago_crimes_2024.csv

```

    
    real	0m4.854s
    user	0m2.935s
    sys	0m1.610s



```python
# 200k Lines -> 4 thread
!time ./rust_csv_to_json ./chicago_crimes_2024.csv 4

```

    
    real	0m4.850s
    user	0m2.931s
    sys	0m1.686s



```python
# 200k Lines -> 6 thread
!time ./rust_csv_to_json ./chicago_crimes_2024.csv 6
```

    
    real	0m3.019s
    user	0m2.365s
    sys	0m1.629s



```python
# 1M Lines -> 1 thread
!time ./rust_csv_to_json ./chicago_crimes_2020_to_2024.csv 1
```

    
    real	0m34.775s
    user	0m14.533s
    sys	0m8.583s



```python
# 1M Lines -> 6 thread
!time ./rust_csv_to_json ./chicago_crimes_2020_to_2024.csv 6
```

    
    real	0m36.062s
    user	0m15.996s
    sys	0m8.908s



```python
# 1M Lines -> 8 thread
!time ./rust_csv_to_json ./chicago_crimes_2020_to_2024.csv 8
```

    
    real	0m31.712s
    user	0m15.123s
    sys	0m9.084s


# Parallelization Channels


```python
!chmod +x ./rust_csv_to_json
```


```python
# 200k Lines -> 1 thread
!time ./rust_csv_to_json ./chicago_crimes_2024.csv 1

```

    
    real	0m4.812s
    user	0m3.540s
    sys	0m1.541s



```python
# 200k Lines -> 6 thread
!time ./rust_csv_to_json ./chicago_crimes_2024.csv 6
```

    
    real	0m3.060s
    user	0m2.711s
    sys	0m1.506s



```python
# 200k Lines -> 8 thread
!time ./rust_csv_to_json ./chicago_crimes_2024.csv 8
```

    
    real	0m2.965s
    user	0m2.381s
    sys	0m1.624s



```python
# 1M Lines -> 1 thread
!time ./rust_csv_to_json ./chicago_crimes_2020_to_2024.csv 1
```

    
    real	0m24.713s
    user	0m22.822s
    sys	0m2.474s



```python
# 1M Lines -> 6 threads
!time ./rust_csv_to_json ./chicago_crimes_2020_to_2024.csv 6
```

    
    real	0m25.715s
    user	0m16.469s
    sys	0m7.650s



```python
# 1M Lines -> 8 threads
!time ./rust_csv_to_json ./chicago_crimes_2020_to_2024.csv 8
```

    
    real	0m26.961s
    user	0m14.892s
    sys	0m8.069s


# Rayon


```python
!chmod +x ./rust_csv_to_json
```


```python
# 200k Lines ->
!time ./rust_csv_to_json ./chicago_crimes_2024.csv

```

    
    real	0m3.217s
    user	0m2.660s
    sys	0m1.707s



```python
# 1M Lines
!time ./rust_csv_to_json ./chicago_crimes_2020_to_2024.csv
```

    
    real	0m25.581s
    user	0m17.412s
    sys	0m10.665s


# **Rayon (Batches)**



```python
!chmod +x ./rust_csv_to_json
```




```python
# 200k Lines. Batch Size -> 100
!time ./rust_csv_to_json ./chicago_crimes_2024.csv 100
```

    
    real	0m2.789s
    user	0m2.408s
    sys	0m1.714s



```python
# 200k Lines. Batch Size -> 500
!time ./rust_csv_to_json ./chicago_crimes_2024.csv 500
```

    
    real	0m2.822s
    user	0m2.384s
    sys	0m1.752s



```python
# 1M Lines. Batch Size -> 100
!time ./rust_csv_to_json ./chicago_crimes_2020_to_2024.csv 100
```

    
    real	0m24.255s
    user	0m14.689s
    sys	0m9.733s



```python
# 1M Lines. Batch Size -> 500
!time ./rust_csv_to_json ./chicago_crimes_2020_to_2024.csv 500
```

    
    real	0m25.853s
    user	0m14.507s
    sys	0m9.734s



```python
# 1M Lines. Batch Size -> 1000
!time ./rust_csv_to_json ./chicago_crimes_2020_to_2024.csv 1000
```

    
    real	0m35.495s
    user	0m14.874s
    sys	0m19.495s


# **Rayon Crossbeam**


```python
!chmod +x ./rust_csv_to_json
```


```python
# 200k Lines. Batch Size -> 100 -> bound size 5
!time ./rust_csv_to_json ./chicago_crimes_2024.csv 100
```

    
    real	0m2.676s
    user	0m3.707s
    sys	0m0.682s



```python
# 1M Lines. Batch Size -> 100 -> bound size 5
!time ./rust_csv_to_json ./chicago_crimes_2020_to_2024.csv 100
```

    
    real	0m18.630s
    user	0m23.304s
    sys	0m2.731s



```python
# 1M Lines. Batch Size -> 500 -> bound size 5
!time ./rust_csv_to_json ./chicago_crimes_2020_to_2024.csv 500
```

    
    real	0m15.708s
    user	0m19.351s
    sys	0m3.667s



```python
# 1M Lines. Batch Size -> 500 -> bound size 20
!time ./rust_csv_to_json ./chicago_crimes_2020_to_2024.csv 500 20
```

    
    real	0m15.032s
    user	0m18.924s
    sys	0m3.725s


# **Channels vs Rayon Batches vs Rayon Crossbeam vs No vec storage (No Ram Usage) Single Threaded**

### 2M Records



```python
!chmod +x ./gc_parallel_channels
!chmod +x ./gc_rayon_batching
!chmod +x ./gc_rayon_crossbeam
!chmod +x ./gc_no_vec_mem
```


```python
# Channels
# 1 thread
!time ./gc_parallel_channels ./chicago_crimes_2015_to_2024.csv 1
```

    /bin/bash: line 1: 62787 Killed                  ./gc_parallel_channels ./chicago_crimes_2015_to_2024.csv 1
    
    real	2m45.024s
    user	0m32.613s
    sys	0m38.462s



```python
# Channels
# 4 thread
!time ./gc_parallel_channels ./chicago_crimes_2015_to_2024.csv 4
```

    /bin/bash: line 1: 63658 Killed                  ./gc_parallel_channels ./chicago_crimes_2015_to_2024.csv 4
    
    real	1m32.757s
    user	0m24.480s
    sys	0m25.042s



```python
# Rayon Batches
# 100 batches
!time ./gc_rayon_batching ./chicago_crimes_2015_to_2024.csv 25
```

    ^C



```python
# Rayon Batches
# 1000 batches
!time ./gc_rayon_batching ./chicago_crimes_2015_to_2024.csv 1000
```

    /bin/bash: line 1: 64013 Killed                  ./gc_rayon_batching ./chicago_crimes_2015_to_2024.csv 1000
    
    real	1m24.764s
    user	0m23.497s
    sys	0m19.090s



```python
# Rayon Crossbeam
# 100 batches
# 20 bounded
!time ./gc_rayon_crossbeam ./chicago_crimes_2015_to_2024.csv 500 20
```

    
    real	0m52.243s
    user	0m52.847s
    sys	0m6.397s



```python
# Rayon Crossbeam
# 1000 batches
# 20 bounded
!time ./gc_rayon_crossbeam ./chicago_crimes_2015_to_2024.csv 1000 100
```

    
    real	0m49.903s
    user	0m46.225s
    sys	0m10.936s



```python
# Rayon Crossbeam
# 1000 batches
# 20 bounded
!time ./gc_rayon_crossbeam ./chicago_crimes_2015_to_2024.csv 5000 20
```

    
    real	0m53.201s
    user	0m45.722s
    sys	0m11.350s



```python
# Rayon Crossbeam
# 1000 batches
# 20 bounded
!time ./gc_no_vec_mem ./chicago_crimes_2015_to_2024.csv
```

    
    real	0m33.026s
    user	0m25.865s
    sys	0m4.573s



```python

```
