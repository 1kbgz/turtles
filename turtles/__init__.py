__version__ = "0.1.0"

# Import all classes from the Rust extension
from .turtles import (
    GuillochePattern,
    HorizontalSpirograph,
    SphericalSpirograph,
    VerticalSpirograph,
)

__all__ = (
    "WatchFaceGenerator",
    "HorizontalSpirograph",
    "VerticalSpirograph",
    "SphericalSpirograph",
    "GuillochePattern",
)


class WatchFaceGenerator:
    def __init__(self, radius):
        self.guilloche = GuillochePattern(radius=radius)

    def add_outline(self):
        outline = HorizontalSpirograph(radius=self.guilloche.radius, radius_ratio=0.001, point_distance=0, rotations=1, resolution=360)
        self.guilloche.add_layer(outline)

    def add_hour_marks(self, mark_length=2.0, mark_width=0.5):
        """Add uniform hour marks around the watch face."""
        # TODO
        ...

    def add_minute_marks(self, mark_length=1.0, mark_width=0.3):
        """Add uniform minute marks around the watch face."""
        # TODO
        ...

    def add_subdial(self, hour, minute, radius):
        """Add a subdial at the specified hour and minute position."""
        # TODO
        ...

    def add_window(self, hour, minute, width, height, angle):
        """Add a date/day/etc window at the specified hour and minute position., with given width, height. Window will be vertically square unless angle is specified."""
        # TODO
        ...

    def add_hole(self, hour, minute, radius):
        """Add a hole at the specified hour and minute position."""
        # TODO
        ...

    def add_layer(self, layer):
        self.guilloche.add_layer(layer)

    def generate(self):
        self.guilloche.generate()

    def to_svg(self, filename):
        self.guilloche.to_svg(filename)

    def to_stl(self, filename):
        self.guilloche.to_stl(filename)

    def to_step(self, filename):
        self.guilloche.to_step(filename)
