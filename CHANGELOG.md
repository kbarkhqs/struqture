# Changelog

This changelog track changes to the struqture project starting at version v1.0.0

## Unreleased

## 1.12.1

* Updated to struqture 2.0 v2.0.0-alpha.11.
* Moved the struqture 2.0 conversion code from struqture to struqture-py, thereby removing the struqture 2.0 dependency of struqture.

## 1.12.0

* Updated to pyo3 0.23 (includes updating to qoqo-calculator 1.5.0 and struqture 2.0.0-alpha.10).
* Updated to new struqture 2.0 naming (Qubit -> Pauli).
* Switched from `from_struqture_2` to `from_json_struqture_2` in the `unstable_struqture_2_import` feature.
* Added qoqo/.cargo/config file with aarch64 and x86_64 targets for macos.

## 1.11.0 - 1.11.1

* Updated to struqture 2.0.0-alpha.7.
* Updated dependencies: jsonschema (0.18 -> 0.28), ndarray (0.15 -> 0.16), thiserror (1.0 -> 2.0), itertools (0.13 -> 0.14), qoqo-calculator (1.2 -> 1.4).
* Updated minimum supported Rust version from 1.57 to 1.76.
* Updated minimum supported Python version from 3.8 to 3.9.

## 1.10.1

* Fixed a build issue in 1.10.0.

## 1.10.0

* Updated to pyo3 0.22 and python 3.13.

## 1.9.2

* Fixed a bug when creating a Product from a bad JSON

## 1.9.0 - 1.9.1

* Added methods to convert from struqture 2.0.0-alpha.3

## 1.8.0

* Added IDE hint support.

## 1.7.1

* Fixed versioning bug

## 1.7.0

* Updated to pyo3 0.21

## 1.6.2

* Updated VersionMissmatch error message

## 1.6.1

* Updated Cargo.lock (particularly mio 0.8.10->0.8.11)

## 1.6.0

* Add optional feature `indexed_map_iterators` switching internal HashMaps to `indexmap` implementation. Using this feature will change the type of iterators returned by `keys`, `values` and `iter` methods.
* Switching Python interface to using `indexed_map_iterators` by default. This emulates the usual Python behavior of returning the elements of dictionary-like objects in the order of insertion.

## 1.5.2

* Updated to pyo3 0.20

## 1.5.1

* Removed print statement from __init__.py file.

## 1.5.0

* Added remap_modes function to fermionic and bosonic indices for the pyo3 interface.

## 1.4.1

* Added remap_modes function to fermionic and bosonic indices in pure Rust.

## 1.4.0

* Fixed bug in Jordan-Wigner transformation for FermionHamiltonian and FermionHamiltonianSystem.
* Added MixedPlusMinusProduct, MixedPlusMinusOperator to mod.rs in struqture-py/src/mixed_systems (fixed import error).
* Added conversion from SpinHamiltonian(System) to PlusMinusOperator.
* Added support for jsonschema in struqture and struqture-py.

## 1.3.1

* Fixed bug allowing the construction of Hermitian operator products with annihilator index lower than creator index when there are leading equal indices.
* Updated pyo3 dependency to 0.19

## 1.3.0

* Added Jordan-Wigner transform to both struqture and struqture-py.

## 1.2.0

* Added MixedPlusMinusProduct and MixedPlusMinusOperator to both struqture and struqture-py.

## 1.1.1

* Fixed failing group when system and noise have the same number of current spins or modes put one of them has not fixed number of spins/modes.

## 1.1.0

* Added support for sigma +, sigma - and sigma z spin basis

## 1.0.1

* Updated to pyo3 0.18 and test-case 3.0

## 1.0.0

* Added `noise_get` and `system_get` getters for all OpenSystems in python interface
* Added a number of particles check to MixedHamiltonianSystem, MixedSystem and MixedLindbladNoiseSystem
