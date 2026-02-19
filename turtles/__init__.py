__version__ = "0.1.0"

# Import all classes from the Rust extension
from .turtles import (
    CuttingBit,
    DiamantLayer,
    DraperieLayer,
    FlinqueLayer,
    HuitEightLayer,
    LimaconLayer,
    PaonLayer,
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
    "DraperieLayer",
    "FlinqueLayer",
    "HuitEightLayer",
    "LimaconLayer",
    "PaonLayer",
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
        radius: float = None,
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
            radius: Radius of the flinqué pattern. Defaults to watch face radius.
            hour: Hour position for center (1-12, default 12 = centered).
            minute: Minute position for center (0-59).
            distance: Distance from center (0 = centered on watch face).
            num_petals: Number of radial petals/segments.
            num_waves: Number of concentric wave rings.
            wave_amplitude: Amplitude of the wave pattern.
            wave_frequency: Frequency of fine ripple texture.
            inner_radius_ratio: Inner radius as fraction of outer radius.
        """
        if radius is None:
            radius = self.radius
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

    def add_flinque_layer(self, layer):
        """Add a pre-configured FlinqueLayer to the watch face.

        Args:
            layer: A FlinqueLayer instance.
        """
        self._watch_face.add_flinque_layer(layer)

    def add_diamant(
        self,
        num_circles: int = 72,
        circle_radius: float = None,
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
            circle_radius: Radius of each individual circle. Defaults to watch face radius.
            hour: Hour position for center (1-12, default 12 = centered).
            minute: Minute position for center (0-59).
            distance: Distance from center (0 = centered on watch face).
            resolution: Number of points per circle.
        """
        if circle_radius is None:
            circle_radius = self.radius
        self._watch_face.add_diamant_at_clock(
            num_circles=num_circles,
            circle_radius=circle_radius,
            hour=hour,
            minute=minute,
            distance=distance,
            resolution=resolution,
        )

    def add_diamant_layer(self, layer):
        """Add a pre-configured DiamantLayer to the watch face.

        Args:
            layer: A DiamantLayer instance.
        """
        self._watch_face.add_diamant_layer(layer)

    def add_limacon(
        self,
        num_curves: int = 72,
        base_radius: float = None,
        amplitude: float = None,
        hour: int = 12,
        minute: int = 0,
        distance: float = 0.0,
        resolution: int = 360,
    ):
        """Add a limaçon guilloché pattern.

        The limaçon pattern is formed using limaçon curves that are tangent to the center,
        rotated around the center. The overlapping curves create intricate patterns.

        Args:
            num_curves: Number of limaçon curves to draw (more = denser pattern).
            base_radius: Base radius of the limaçon curves. Defaults to watch face radius.
            amplitude: Amplitude of the limaçon curves. Defaults to watch face radius.
            hour: Hour position for center (1-12, default 12 = centered).
            minute: Minute position for center (0-59).
            distance: Distance from center (0 = centered on watch face).
            resolution: Number of points per curve.
        """
        if base_radius is None:
            base_radius = self.radius
        if amplitude is None:
            amplitude = self.radius
        self._watch_face.add_limacon_at_clock(
            num_curves=num_curves,
            base_radius=base_radius,
            amplitude=amplitude,
            hour=hour,
            minute=minute,
            distance=distance,
            resolution=resolution,
        )

    def add_limacon_layer(self, layer):
        """Add a pre-configured LimaconLayer to the watch face.

        Args:
            layer: A LimaconLayer instance.
        """
        self._watch_face.add_limacon_layer(layer)

    def add_draperie(
        self,
        num_rings: int = 96,
        base_radius: float = None,
        radius_step: float = 0.44,
        wave_frequency: float = 12.0,
        phase_shift: float = None,
        phase_oscillations: float = 2.5,
        hour: int = 12,
        minute: int = 0,
        distance: float = 0.0,
        resolution: int = 1500,
        phase_exponent: int = 3,
        wave_exponent: int = 1,
        circular_phase: float = 2.0,
    ):
        """Add a draperie (drapery) guilloché pattern.

        The draperie pattern creates flowing, fabric-like folds through concentric
        wavy rings whose phase oscillates sinusoidally from the innermost to the
        outermost ring. Amplitude is automatically computed so that adjacent rings
        never overlap.

        Args:
            num_rings: Number of concentric rings (more = denser).
            base_radius: Centre of the ring band in mm. Defaults to watch face radius.
            radius_step: Radial spacing between ring centres.
            wave_frequency: Number of wave undulations per revolution.
            phase_shift: Peak angular oscillation in radians (default: π/12 ≈ 15°).
            phase_oscillations: Number of full sinusoidal phase cycles.
            hour: Hour position for center (1-12, default 12 = centered).
            minute: Minute position for center (0-59).
            distance: Distance from center (0 = centered on watch face).
            resolution: Number of points per ring.
            phase_exponent: Exponent for sin-power phase (only when circular_phase=0).
            wave_exponent: Exponent for the wave shape (1 = sinusoidal, 3 = softer crests).
            circular_phase: Dome-shaped phase exponent; 0 disables, 2.0 = rounded folds (default).
        """
        if base_radius is None:
            base_radius = self.radius
        self._watch_face.add_draperie_at_clock(
            hour=hour,
            minute=minute,
            distance=distance,
            num_rings=num_rings,
            base_radius=base_radius,
            radius_step=radius_step,
            wave_frequency=wave_frequency,
            phase_shift=phase_shift,
            phase_oscillations=phase_oscillations,
            resolution=resolution,
            phase_exponent=phase_exponent,
            wave_exponent=wave_exponent,
            circular_phase=circular_phase,
        )

    def add_draperie_layer(self, layer):
        """Add a pre-configured DraperieLayer to the watch face.

        Args:
            layer: A DraperieLayer instance.
        """
        self._watch_face.add_draperie_layer(layer)

    def add_draperie_sharp(
        self,
        num_rings: int = 96,
        base_radius: float = None,
        radius_step: float = 0.44,
        wave_frequency: float = 12.0,
        phase_shift: float = None,
        phase_oscillations: float = 2.5,
        hour: int = 12,
        minute: int = 0,
        distance: float = 0.0,
        resolution: int = 1500,
        wave_exponent: int = 1,
    ):
        """Add a sharp-angled draperie guilloché pattern.

        Identical to ``add_draperie`` but disables circular phase and uses
        phase_exponent=1 for sharp V-shaped fold lines.

        Args:
            num_rings: Number of concentric rings (more = denser).
            base_radius: Centre of the ring band in mm.
            radius_step: Radial spacing between ring centres.
            wave_frequency: Number of wave undulations per revolution.
            phase_shift: Peak angular oscillation in radians (default: π/12 ≈ 15°).
            phase_oscillations: Number of full sinusoidal phase cycles.
            hour: Hour position for center (1-12, default 12 = centered).
            minute: Minute position for center (0-59).
            distance: Distance from center (0 = centered on watch face).
            resolution: Number of points per ring.
            wave_exponent: Exponent for the wave shape (1 = sinusoidal, 3 = softer crests).
        """
        self.add_draperie(
            num_rings=num_rings,
            base_radius=base_radius,
            radius_step=radius_step,
            wave_frequency=wave_frequency,
            phase_shift=phase_shift,
            phase_oscillations=phase_oscillations,
            hour=hour,
            minute=minute,
            distance=distance,
            resolution=resolution,
            phase_exponent=1,
            wave_exponent=wave_exponent,
            circular_phase=0.0,
        )

    def add_paon(
        self,
        num_lines: int = 500,
        radius: float = None,
        amplitude: float = 0.1,
        wave_frequency: float = 15.0,
        phase_rate: float = 9.0,
        hour: int = 12,
        minute: int = 0,
        distance: float = 0.0,
        resolution: int = 800,
        n_harmonics: int = 5,
        fan_angle: float = 3.0,
        vanishing_point: float = 0.2,
    ):
        """Add a paon (peacock) guilloché pattern.

        Lines fan outward from 6 o'clock, each zigzagging perpendicular to its
        travel direction.  Phase offsets between neighbouring lines create the
        characteristic peacock-feather arch bands.

        Args:
            num_lines: Number of fan lines (more = denser pattern).
            radius: Radius of the circular clipping region in mm. Defaults to watch face radius.
            amplitude: Perpendicular oscillation amplitude in mm.
            wave_frequency: Number of zigzag cycles per line.
            phase_rate: Phase change rate across fan (controls arch band count).
            hour: Hour position for center (1-12, default 12 = centered).
            minute: Minute position for center (0-59).
            distance: Distance from center (0 = centered on watch face).
            resolution: Number of sample points per line.
            n_harmonics: 0=pure sine (smooth arches), 1+=triangle-wave (sharper cusps).
            fan_angle: Total angular spread of the fan in radians (~1.4 = 80°).
            vanishing_point: VP distance below circle bottom (fraction of diameter).
        """
        if radius is None:
            radius = self.radius
        self._watch_face.add_paon_at_clock(
            hour=hour,
            minute=minute,
            distance=distance,
            num_lines=num_lines,
            radius=radius,
            amplitude=amplitude,
            wave_frequency=wave_frequency,
            phase_rate=phase_rate,
            resolution=resolution,
            n_harmonics=n_harmonics,
            fan_angle=fan_angle,
            vanishing_point=vanishing_point,
        )

    def add_paon_layer(self, layer):
        """Add a pre-configured PaonLayer to the watch face.

        Args:
            layer: A PaonLayer instance.
        """
        self._watch_face.add_paon_layer(layer)

    def add_huiteight(
        self,
        num_curves: int = 72,
        scale: float = None,
        hour: int = 12,
        minute: int = 0,
        distance: float = 0.0,
        resolution: int = 360,
        num_clusters: int = 0,
        cluster_spread: float = 0.0,
    ):
        """Add a huit-eight (figure-eight) guilloché pattern.

        The huit-eight pattern is formed by drawing lemniscate curves (figure-eights)
        that pass through the centre, rotated around the centre.  The overlapping
        lemniscates create an intricate woven mesh pattern.

        Args:
            num_curves: Number of figure-eight curves (more = denser mesh).
            scale: Half-width of each lemniscate. Defaults to watch face radius.
            hour: Hour position for center (1-12, default 12 = centered).
            minute: Minute position for center (0-59).
            distance: Distance from center (0 = centered on watch face).
            resolution: Number of points per curve.
            num_clusters: Number of clusters to group curves into (0 = uniform).
            cluster_spread: Angular spread within each cluster in radians (0.0 = auto).
        """
        if scale is None:
            scale = self.radius
        self._watch_face.add_huiteight_at_clock(
            num_curves=num_curves,
            scale=scale,
            hour=hour,
            minute=minute,
            distance=distance,
            resolution=resolution,
            num_clusters=num_clusters,
            cluster_spread=cluster_spread,
        )

    def add_huiteight_layer(self, layer):
        """Add a pre-configured HuitEightLayer to the watch face.

        Args:
            layer: A HuitEightLayer instance.
        """
        self._watch_face.add_huiteight_layer(layer)

    def add(self, layer):
        """Add a spirograph, flinque, diamant, draperie, huiteight, limacon, or paon layer."""
        if isinstance(layer, FlinqueLayer):
            self._watch_face.add_flinque_layer(layer)
        elif isinstance(layer, DiamantLayer):
            self._watch_face.add_diamant_layer(layer)
        elif isinstance(layer, DraperieLayer):
            self._watch_face.add_draperie_layer(layer)
        elif isinstance(layer, HuitEightLayer):
            self._watch_face.add_huiteight_layer(layer)
        elif isinstance(layer, LimaconLayer):
            self._watch_face.add_limacon_layer(layer)
        elif isinstance(layer, PaonLayer):
            self._watch_face.add_paon_layer(layer)
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
