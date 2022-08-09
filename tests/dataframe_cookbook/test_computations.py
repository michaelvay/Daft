from daft.expressions import col
from tests.conftest import assert_df_equals
from tests.dataframe_cookbook.conftest import (
    parametrize_service_requests_csv_daft_df,
    parametrize_service_requests_csv_repartition,
)


@parametrize_service_requests_csv_repartition
@parametrize_service_requests_csv_daft_df
def test_add_one_to_column(daft_df, service_requests_csv_pd_df, repartition_nparts):
    """Creating a new column that is derived from (1 + other_column) and retrieving the top N results"""
    daft_df = daft_df.repartition(repartition_nparts).with_column("unique_key_mod", col("Unique Key") + 1)
    service_requests_csv_pd_df["unique_key_mod"] = service_requests_csv_pd_df["Unique Key"] + 1
    assert_df_equals(daft_df, service_requests_csv_pd_df)


@parametrize_service_requests_csv_repartition
@parametrize_service_requests_csv_daft_df
def test_difference_cols(daft_df, service_requests_csv_pd_df, repartition_nparts):
    """Creating a new column that is derived from 2 other columns and retrieving the top N results"""
    daft_df = daft_df.repartition(repartition_nparts).with_column(
        "unique_key_mod", col("Unique Key") - col("Unique Key")
    )
    service_requests_csv_pd_df["unique_key_mod"] = (
        service_requests_csv_pd_df["Unique Key"] - service_requests_csv_pd_df["Unique Key"]
    )
    assert_df_equals(daft_df, service_requests_csv_pd_df)