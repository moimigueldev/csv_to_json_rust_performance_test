# Performance Rankings with Percentage Improvement

## 200k Records: Best to Worst
1. **`parallel_file_batching`**: **3.725 seconds**  
   - _(Baseline)_
2. **`rayon_crossbeam`**: **7.617 seconds**  
   - **104.5% slower** than `parallel_file_batching`.
3. **`no_vec_mem`**: **8.150 seconds**  
   - **7% slower** than `rayon_crossbeam`.
4. **`parallel_channels`**: **10.802 seconds**  
   - **32.5% slower** than `no_vec_mem`.
5. **`no_vec_mem_no_nested_loops`**: **11.479 seconds**  
   - **6.3% slower** than `parallel_channels`.
6. **`rayon_solution`**: **14.289 seconds**  
   - **24.5% slower** than `no_vec_mem_no_nested_loops`.

---

## 2M Records: Best to Worst
1. **`parallel_file_batching`**: **8.213 seconds**  
   - _(Baseline)_
2. **`rayon_crossbeam`**: **18.174 seconds**  
   - **121.2% slower** than `parallel_file_batching`.
3. **`no_vec_mem`**: **19.128 seconds**  
   - **5.3% slower** than `rayon_crossbeam`.
4. **`no_vec_mem_no_nested_loops`**: **22.705 seconds**  
   - **18.7% slower** than `no_vec_mem`.
5. **`parallel_channels`**: **23.425 seconds**  
   - **3.2% slower** than `no_vec_mem_no_nested_loops`.
6. **`rayon_solution`**: **31.218 seconds**  
   - **33.3% slower** than `parallel_channels`.
