import pytest  # type: ignore

import grimp


@pytest.mark.parametrize(
    "package_name",
    ("django", "flask", "requests", "sqlalchemy", "google.cloud.audit"),
)
def test_build_graph_on_real_package(package_name, snapshot):
    graph = grimp.build_graph(package_name, cache_dir=None)
    imports = graph.find_matching_direct_imports(import_expression="** -> **")
    assert imports == snapshot
