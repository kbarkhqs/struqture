// Copyright © 2021-2023 HQS Quantum Simulations GmbH. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except
// in compliance with the License. You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software distributed under the
// License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either
// express or implied. See the License for the specific language governing permissions and
// limitations under the License.

use crate::fermions::FermionLindbladNoiseOperatorWrapper;
use crate::spins::PlusMinusProductWrapper;
use bincode::deserialize;
use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyByteArray;
use qoqo_calculator_pyo3::CalculatorComplexWrapper;
use struqture::mappings::JordanWignerSpinToFermion;
use struqture::spins::{PlusMinusLindbladNoiseOperator, SpinLindbladNoiseOperator};
use struqture::OperateOnDensityMatrix;
use struqture_py_macros::{mappings, noisy_system_wrapper};

use super::SpinLindbladNoiseOperatorWrapper;
#[cfg(feature = "json_schema")]
use struqture::{MinSupportedVersion, STRUQTURE_VERSION};

/// These are representations of noisy systems of spins.
///
/// In a PlusMinusLindbladNoiseOperator is characterized by a SpinLindbladNoiseOperator to represent the hamiltonian of the spin system, and an optional number of spins.
///
/// Examples
/// --------
///
/// .. code-block:: python
///
///     import numpy.testing as npt
///     from qoqo_calculator_pyo3 import CalculatorComplex
///     from struqture_py.spins import PlusMinusLindbladNoiseOperator, PlusMinusProduct
///
///     slns = PlusMinusLindbladNoiseOperator()
///     dp = PlusMinusProduct().z(0).plus(1)
///     slns.add_operator_product((dp, dp), 2.0)
///     npt.assert_equal(slns.get((dp, dp)), CalculatorComplex(2))
///     npt.assert_equal(slns.keys(), [(dp, dp)])
///
#[pyclass(name = "PlusMinusLindbladNoiseOperator", module = "struqture_py.spins")]
#[derive(Clone, Debug, PartialEq, Default)]
pub struct PlusMinusLindbladNoiseOperatorWrapper {
    /// Internal storage of [struqture::spins::PlusMinusLindbladNoiseOperator]
    pub internal: PlusMinusLindbladNoiseOperator,
}

#[mappings(JordanWignerSpinToFermion)]
#[noisy_system_wrapper(OperateOnDensityMatrix, Calculus)]
impl PlusMinusLindbladNoiseOperatorWrapper {
    /// Create a new PlusMinusLindbladNoiseOperator.
    ///
    /// Returns:
    ///     self: The new PlusMinusLindbladNoiseOperator with the input number of spins.
    #[new]
    pub fn new() -> Self {
        Self {
            internal: PlusMinusLindbladNoiseOperator::new(),
        }
    }

    /// Separate self into an operator with the terms of given number of spins (left and right) and an operator with the remaining operations.
    ///
    /// Args
    ///     number_spins_left (int): Number of spin to filter for in the left key.
    ///     number_spins_right (int): Number of spin to filter for in the right key.
    ///
    /// Returns
    ///     Tuple[PlusMinusLindbladNoiseOperator, PlusMinusLindbladNoiseOperator]: Operator with the noise terms where number_spins (left and right) matches the number of spins the operator product acts on and Operator with all other contributions.
    ///
    /// Raises:
    ///     ValueError: Error in adding terms to return values.
    pub fn separate_into_n_terms(
        &self,
        number_spins_left: usize,
        number_spins_right: usize,
    ) -> PyResult<(
        PlusMinusLindbladNoiseOperatorWrapper,
        PlusMinusLindbladNoiseOperatorWrapper,
    )> {
        let result = self
            .internal
            .separate_into_n_terms(number_spins_left, number_spins_right)
            .map_err(|err| PyValueError::new_err(format!("{:?}", err)))?;
        Ok((
            PlusMinusLindbladNoiseOperatorWrapper { internal: result.0 },
            PlusMinusLindbladNoiseOperatorWrapper { internal: result.1 },
        ))
    }

    /// Convert a SpinLindbladNoiseOperator into a PlusMinusLindbladNoiseOperator.
    ///
    /// Args:
    ///     value (SpinLindbladNoiseOperator): The SpinLindbladNoiseOperator to create the PlusMinusLindbladNoiseOperator from.
    ///
    /// Returns:
    ///     PlusMinusLindbladNoiseOperator: The operator created from the input SpinLindbladNoiseOperator.
    ///
    /// Raises:
    ///     ValueError: Could not create SpinLindbladNoiseOperator from input.
    #[staticmethod]
    pub fn from_spin_noise_system(
        value: Py<PyAny>,
    ) -> PyResult<PlusMinusLindbladNoiseOperatorWrapper> {
        let system = SpinLindbladNoiseOperatorWrapper::from_pyany(value)
            .map_err(|err| PyValueError::new_err(format!("{:?}", err)))?;
        Ok(PlusMinusLindbladNoiseOperatorWrapper {
            internal: PlusMinusLindbladNoiseOperator::from(system.clone()),
        })
    }

    /// Convert a PlusMinusLindbladNoiseOperator into a SpinLindbladNoiseOperator.
    ///
    /// Returns:
    ///     SpinLindbladNoiseOperator: The operator created from the input PlusMinusLindbladNoiseOperator and optional number of spins.
    ///
    /// Raises:
    ///     ValueError: Could not create SpinLindbladNoiseOperator from PlusMinusLindbladNoiseOperator.
    pub fn to_spin_noise_system(&self) -> PyResult<SpinLindbladNoiseOperatorWrapper> {
        let result: SpinLindbladNoiseOperator =
            SpinLindbladNoiseOperator::from(self.internal.clone());
        Ok(SpinLindbladNoiseOperatorWrapper { internal: result })
    }
}
