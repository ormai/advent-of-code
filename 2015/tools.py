from collections.abc import Sequence
from typing import Generator, Any


def window(iterable: Sequence[Any], size: int, advance=1) -> Generator[Sequence[Any]]:
    """https://ziglang.org/documentation/master/std/#std.mem.window"""
    for i in range(0, len(iterable) - size, advance):
        yield iterable[i : i + size]
