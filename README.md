# BioMC Postprocessor API



## Project Overview
The **BioMC Postprocessor API** is a Rust module designed to facilitate interaction with post-processing simulation results via the BiomC API.  
It also includes **PyO3 bindings** to provide a high-level Python API for end users.


### Structure 



- **Core**
    - **`datamodel`**: Implementation to read HDF5 files, ensuring efficient data extraction.
    - **`process`**: Core for data manipulations and computations, enabling transformation of datasets.
    - **`End-User API`**: Simplified methods for data extraction, processing, and visualization.
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

This tool is under [Apache License, Version 2.0](./LICENSE) *(Apache-2.0)*
