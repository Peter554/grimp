from __future__ import annotations
from typing import List, Optional, Sequence, Set, Tuple

from grimp.application.ports.graph import DetailedImport
from grimp.domain.analysis import PackageDependency
from grimp.domain.valueobjects import Layer
from grimp import _rustgrimp as rust  # type: ignore[attr-defined]

from . import graph as python_graph


class ImportGraph(python_graph.ImportGraph):
    """
    Rust-backed implementation of the ImportGraph.
    """

    def __init__(self) -> None:
        super().__init__()
        self._rustgraph = rust.Graph()

    @property
    def modules(self) -> Set[str]:
        return super().modules

    def add_module(self, module: str, is_squashed: bool = False) -> None:
        self._rustgraph.add_module(module, is_squashed)
        super().add_module(module, is_squashed)

    def remove_module(self, module: str) -> None:
        self._rustgraph.remove_module(module)
        super().remove_module(module)

    def squash_module(self, module: str) -> None:
        super().squash_module(module)
        # TODO raise ModuleNotPresent if not in graph.
        self._rustgraph.squash_module(module)

    def is_module_squashed(self, module: str) -> bool:
        self._rustgraph.is_module_squashed(module)
        return super().is_module_squashed(module)

    def add_import(
        self,
        *,
        importer: str,
        imported: str,
        line_number: Optional[int] = None,
        line_contents: Optional[str] = None,
    ) -> None:
        self._rustgraph.add_import(
            importer=importer,
            imported=imported,
            line_number=line_number,
            line_contents=line_contents,
        )
        return super().add_import(
            importer=importer,
            imported=imported,
            line_number=line_number,
            line_contents=line_contents,
        )

    def remove_import(self, *, importer: str, imported: str) -> None:
        self._rustgraph.remove_import(importer=importer, imported=imported)
        return super().remove_import(importer=importer, imported=imported)

    def count_imports(self) -> int:
        return super().count_imports()

    def find_children(self, module: str) -> Set[str]:
        return super().find_children(module)

    def find_descendants(self, module: str) -> Set[str]:
        return super().find_descendants(module)

    def direct_import_exists(
        self, *, importer: str, imported: str, as_packages: bool = False
    ) -> bool:
        return super().direct_import_exists(
            importer=importer, imported=imported, as_packages=as_packages
        )

    def find_modules_directly_imported_by(self, module: str) -> Set[str]:
        return super().find_modules_directly_imported_by(module)

    def find_modules_that_directly_import(self, module: str) -> Set[str]:
        return super().find_modules_that_directly_import(module)

    def get_import_details(self, *, importer: str, imported: str) -> List[DetailedImport]:
        return super().get_import_details(importer=importer, imported=imported)

    def find_downstream_modules(self, module: str, as_package: bool = False) -> Set[str]:
        return super().find_downstream_modules(module, as_package)

    def find_upstream_modules(self, module: str, as_package: bool = False) -> Set[str]:
        return super().find_upstream_modules(module, as_package)

    def find_shortest_chain(self, importer: str, imported: str) -> tuple[str, ...] | None:
        return super().find_shortest_chain(importer, imported)

    def find_shortest_chains(
        self, importer: str, imported: str, as_packages: bool = True
    ) -> Set[Tuple[str, ...]]:
        return super().find_shortest_chains(importer, imported, as_packages)

    def chain_exists(self, importer: str, imported: str, as_packages: bool = False) -> bool:
        return super().chain_exists(importer, imported, as_packages)

    def find_illegal_dependencies_for_layers(
        self,
        layers: Sequence[Layer | str | set[str]],
        containers: set[str] | None = None,
    ) -> set[PackageDependency]:
        return super().find_illegal_dependencies_for_layers(layers, containers)