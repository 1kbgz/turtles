__version__ = "0.1.0"

# Import all classes from the Rust extension
from .turtles import (
    FlinqueLayer,
    GuillochePattern,
    HorizontalSpirograph,
    SphericalSpirograph,
    VerticalSpirograph,
)

__all__ = (
    "FlinqueLayer",
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
        outline = HorizontalSpirograph(outer_radius=self.guilloche.radius, radius_ratio=0.001, point_distance=0, rotations=1, resolution=360)
        self.guilloche.add_layer(outline)

    # def add_hour_marks(self, mark_length=2.0, mark_width=0.5):
    #     """Add uniform hour marks around the watch face."""
    #     # TODO
    #     ...

    # def add_minute_marks(self, mark_length=1.0, mark_width=0.3):
    #     """Add uniform minute marks around the watch face."""
    #     # TODO
    #     ...

    def add_subdial(self, hour, minute, radius, subdial_radius):
        """
        Add a subdial at the specified position.
        The position of the subdial center will be calculated by the position of the hour
        hand at the given hour and minute.

        Args:
            hour (int): Hour position (1-12)
            minute (int): Minute position (1-60)
            radius (float): Radius relative to the center of the watch face for the subdial center.
            subdial_radius (float): Radius of the subdial itself.
        """
        # TODO
        ...

    def add_window(self, hour, minute, width, height, angle):
        """Add a date/day/etc window at the specified hour and minute position.
        The center of the window will be calculated according to the position
        of the hour hand at the given hour and minute. It will have the given height
        and width and be vertically aligned unless an angle is specified.

        Args:
            hour (int): Hour position (1-12)
            minute (int): Minute position (1-60)
            width (float): Width of the window.
            height (float): Height of the window.
            angle (float): Angle of the window in degrees.
        """
        # TODO
        ...

    def add_hole(self, hour, minute, radius, hole_radius):
        """Add a hole at the specified hour and minute position.
        The position of the hole center will be calculated by the position of the hour
        hand at the given hour and minute.

        Args:
            hour (int): Hour position (1-12)
            minute (int): Minute position (1-60)
            radius (float): Radius relative to the center of the watch face for the hole center.
            hole_radius (float): Radius of the hole itself.
        """
        # TODO
        ...

    # Textures
    # TODO add_clous_de_paris
    # TODO add_tapisserie
    # TODO add_sunburst
    # TODO add_flinque

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
