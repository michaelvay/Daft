use crate::{array::ListArray, datatypes::logical::TensorArray};

impl TensorArray {
    pub fn data_array(&self) -> &ListArray {
        const DATA_IDX: usize = 0;
        let array = self.physical.children.get(DATA_IDX).unwrap();
        array.list().unwrap()
    }

    pub fn shape_array(&self) -> &ListArray {
        const SHAPE_IDX: usize = 1;
        let array = self.physical.children.get(SHAPE_IDX).unwrap();
        array.list().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        array::{
            ops::DaftCompare, ListArray, StructArray
        },
        datatypes::{
            logical::TensorArray,
            DataType, Field, Int64Array, UInt64Array,
        },
        IntoSeries
    };
    use common_error::DaftResult;

    #[test]
    fn test_tesnor_array() -> DaftResult<()> {
        let list_array = ListArray::new(
            Field::new("data", DataType::List(Box::new(DataType::Int64))),
            Int64Array::from((
                "item",
                Box::new(arrow2::array::Int64Array::from_iter(
                    [Some(0), Some(1), Some(2), Some(0), Some(0), Some(3)].iter(),
                )),
            ))
            .into_series(),
            arrow2::offset::OffsetsBuffer::<i64>::try_from(vec![0, 3, 6])?,
            None,
        )
        .into_series();
        let shapes_array = ListArray::new(
            Field::new("shape", DataType::List(Box::new(DataType::UInt64))),
            UInt64Array::from((
                "item",
                Box::new(arrow2::array::UInt64Array::from_iter(
                    [Some(3), Some(3), Some(1)].iter(),
                )),
            ))
            .into_series(),
            arrow2::offset::OffsetsBuffer::<i64>::try_from(vec![0, 1, 3])?,
            None,
        )
        .into_series();
        let dtype = DataType::Tensor(Box::new(DataType::Int64));
        let struct_array = StructArray::new(
            Field::new("tensor", dtype.to_physical()),
            vec![list_array, shapes_array],
            None,
        );
        let tensor_array =
            TensorArray::new(Field::new(struct_array.name(), dtype.clone()), struct_array);
            
        let coo_sparse_tensor_dtype = DataType::COOSparseTensor(Box::new(DataType::Int64));
        let coo_sparse_tensor = tensor_array.cast(&coo_sparse_tensor_dtype)?;
        let roundtrip_tensor = coo_sparse_tensor.cast(&dtype)?;
        assert!(tensor_array.to_arrow().eq(&roundtrip_tensor.to_arrow()));
        Ok(())
    }
}
