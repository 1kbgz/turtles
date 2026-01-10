__version__ = "0.1.0"

# Import all classes from the Rust extension
from .turtles import (
    CuttingBit,
    DiamantLayer,
    FlinqueLayer,
    RoseEngineConfig,
    RoseEngineLathe,
    RoseEngineLatheRun,
    RosettePattern,
    WatchFace as RustWatchFace,
)

__all__ = (
    "WatchFace",
    "RoseEngineLathe",
    "RoseEngineLatheRun",
    "RoseEngineConfig",
    "CuttingBit",
    "RosettePattern",
    "DiamantLayer",
    "FlinqueLayer",
)


class WatchFace:
    """High-level Python interface for creating watch face designs.

    This class wraps the Rust WatchFace struct, providing a convenient
    Python API for adding dial elements, textures, and exporting.
    """

    def __init__(self, radius: float):
        """Create a new watch face generator.

        Args:
            radius: The radius of the watch face in mm (must be 26-44mm).
        """
        self._watch_face = RustWatchFace(radius=radius)

    @property
    def radius(self) -> float:
        """Get the radius of the watch face."""
        return self._watch_face.radius

    def add_inner(self):
        """Add the inner dial circle."""
        self._watch_face.add_inner()

    def add_outer(self):
        """Add the outer bezel ring."""
        self._watch_face.add_outer()

    def add_center_hole(self):
        """Add the center pinhole for watch hands."""
        self._watch_face.add_center_hole()

    # def add_hour_marks(self, mark_length=2.0, mark_width=0.5):
    #     """Add uniform hour marks around the watch face."""
    #     # TODO
    #     ...

    # def add_minute_marks(self, mark_length=1.0, mark_width=0.3):
    #     """Add uniform minute marks around the watch face."""
    #     # TODO
    #     ...

    def add_subdial(self, hour: int, minute: int, radius: float, subdial_radius: float):
        """Add a subdial at the specified position.

        The position of the subdial center will be calculated by the position of the hour
        hand at the given hour and minute.

        Args:
            hour: Hour position (1-12)
            minute: Minute position (0-59)
            radius: Radius relative to the center of the watch face for the subdial center.
            subdial_radius: Radius of the subdial itself.
        """
        # TODO: Implement subdial rendering
        ...

    def add_window(self, hour: int, minute: int, width: float, height: float, angle: float = 0.0):
        """Add a date/day/etc window at the specified hour and minute position.

        The center of the window will be calculated according to the position
        of the hour hand at the given hour and minute. It will have the given height
        and width and be vertically aligned unless an angle is specified.

        Args:
            hour: Hour position (1-12)
            minute: Minute position (0-59)
            width: Width of the window.
            height: Height of the window.
            angle: Angle of the window in degrees.
        """
        # TODO: Implement window rendering
        ...

    def add_hole(self, hour: int, minute: int, radius: float, hole_radius: float):
        """Add a hole at the specified hour and minute position.

        The position of the hole center will be calculated by the position of the hour
        hand at the given hour and minute.

        Args:
            hour: Hour position (1-12)
            minute: Minute position (0-59)
            radius: Radius relative to the center of the watch face for the hole center.
            hole_radius: Radius of the hole itself.
        """
        self._watch_face.add_hole_at_clock(hour, minute, radius, hole_radius)

    # Textures
    # TODO add_clous_de_paris
    # TODO add_tapisserie
    # TODO add_sunburst
    # TODO add_flinque

    def add_flinque(
        self,
        radius: float,
        hour: int = 12,
        minute: int = 0,
        distance: float = 0.0,
        num_petals: int = 12,
        num_waves: int = 60,
        wave_amplitude: float = 0.8,
        wave_frequency: float = 20.0,
        inner_radius_ratio: float = 0.05,
    ):
        """Add a flinqué (engine-turned) pattern.

        Args:
            radius: Radius of the flinqué pattern.
            hour: Hour position for center (1-12, default 12 = centered).
            minute: Minute position for center (0-59).
            distance: Distance from center (0 = centered on watch face).
            num_petals: Number of radial petals/segments.
            num_waves: Number of concentric wave rings.
            wave_amplitude: Amplitude of the wave pattern.
            wave_frequency: Frequency of fine ripple texture.
            inner_radius_ratio: Inner radius as fraction of outer radius.
        """
        self._watch_face.add_flinque_at_clock(
            radius=radius,
            hour=hour,
            minute=minute,
            distance=distance,
            num_petals=num_petals,
            num_waves=num_waves,
            wave_amplitude=wave_amplitude,
            wave_frequency=wave_frequency,
            inner_radius_ratio=inner_radius_ratio,
        )

    def add_diamant(
        self,
        num_circles: int = 72,
        circle_radius: float = 20.0,
        hour: int = 12,
        minute: int = 0,
        distance: float = 0.0,
        resolution: int = 360,
    ):
        """Add a diamant (diamond) guilloché pattern.

        The diamant pattern is formed by creating equally-sized circles that are
        tangent to the center, rotated around the center. The overlapping circles
        create the characteristic diamond/mesh appearance.

        Args:
            num_circles: Number of circles to draw (more = denser mesh).
            circle_radius: Radius of each individual circle.
            hour: Hour position for center (1-12, default 12 = centered).
            minute: Minute position for center (0-59).
            distance: Distance from center (0 = centered on watch face).
            resolution: Number of points per circle.
        """
        self._watch_face.add_diamant_at_clock(
            num_circles=num_circles,
            circle_radius=circle_radius,
            hour=hour,
            minute=minute,
            distance=distance,
            resolution=resolution,
        )

    def add(self, layer):
        """Add a spirograph, flinque, or diamant layer."""
        if isinstance(layer, FlinqueLayer):
            self._watch_face.add_flinque_layer(layer)
        elif isinstance(layer, DiamantLayer):
            self._watch_face.add_diamant_layer(layer)
        else:
            self._watch_face.add_layer(layer)

    def generate(self):
        """Generate all pattern layers."""
        self._watch_face.generate()

    def to_svg(self, filename: str):
        """Export the watch face to SVG."""
        self._watch_face.to_svg(filename)

    def to_stl(self, filename: str, depth: float = 0.1, base_thickness: float = 2.0):
        """Export the watch face to STL."""
        self._watch_face.to_stl(filename, depth, base_thickness)

    def to_step(self, filename: str, depth: float = 0.1):
        """Export the watch face to STEP."""
        self._watch_face.to_step(filename, depth)
