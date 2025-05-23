# BioMC Postprocessor API



## Project Overview
The **BioMC Postprocessor API** is a Rust API designed to interact with and perform post-processing on simulation results obtained with the [BioMC code](https://github.com/Benncs/BioCMA-MCST).  
It also includes **PyO3 bindings** to provide a high-level Python API for end users.


### Structure 



- **Core**
    - **`datamodel`**: Implementation to read HDF5 results files
    - **`process`**: for data manipulations and computations, enabling transformation of datasets.
    - **`lib`**: End-User API: Simplified methods for data extraction, processing, and visualization.
- **Python Wrap**
    - Rust bindings to be used in Python applications via `PyO3`, integrating Rust functionality with Python.
- **biomc_pp**
    - Python wrapper around Rust bindings, providing access to data processing and plotting functions.
- **examples**
    - Practical examples for integrating core functionality, bindings, and wrappers.


### Features
- Retrieve pre-processed simulation results and parameters.
- Rust and Python integration for versatile usage.


## Authors

- **CASALE Benjamin**

### LICENSE 


This code is under [MIT](./LICENSE) license