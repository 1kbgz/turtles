import os
import tempfile

from turtles import WatchFace
from turtles.turtles import GuillochePattern, HorizontalSpirograph, SphericalSpirograph, VerticalSpirograph


def test_horizontal_spirograph():
    """Test HorizontalSpirograph creation and generation"""
    spiro = HorizontalSpirograph(
        outer_radius=40.0,
        radius_ratio=0.75,
        point_distance=0.6,
        rotations=50,
        resolution=360,
    )
    spiro.generate()
    assert spiro is not None


def test_horizontal_spirograph_invalid_radius():
    """Test that invalid radius raises error"""
    try:
        _ = HorizontalSpirograph(
            outer_radius=50.0,  # Invalid: > 44mm
            radius_ratio=0.75,
            point_distance=0.6,
            rotations=50,
            resolution=360,
        )
        assert False, "Should have raised ValueError"
    except ValueError as e:
        assert "26mm and 44mm" in str(e)


def test_vertical_spirograph():
    """Test VerticalSpirograph creation and generation"""
    spiro = VerticalSpirograph(
        outer_radius=35.0,
        radius_ratio=0.6,
        point_distance=0.5,
        rotations=30,
        resolution=360,
        wave_amplitude=2.0,
        wave_frequency=5.0,
    )
    spiro.generate()
    assert spiro is not None


def test_spherical_spirograph():
    """Test SphericalSpirograph creation and generation"""
    spiro = SphericalSpirograph(
        outer_radius=38.0,
        radius_ratio=0.7,
        point_distance=0.4,
        rotations=40,
        resolution=360,
        dome_height=5.0,
    )
    spiro.generate()
    assert spiro is not None


def test_guilloche_pattern():
    """Test GuillochePattern with multiple layers"""
    pattern = GuillochePattern(radius=38.0)

    h_spiro = HorizontalSpirograph(38.0, 0.75, 0.6, 10, 100)
    v_spiro = VerticalSpirograph(38.0, 0.6, 0.5, 10, 100, wave_amplitude=2.0, wave_frequency=5.0)

    pattern.add_layer(h_spiro)
    pattern.add_layer(v_spiro)
    pattern.generate()

    assert pattern is not None


def test_export_svg():
    """Test SVG export functionality"""
    with tempfile.TemporaryDirectory() as tmpdir:
        spiro = HorizontalSpirograph(40.0, 0.75, 0.6, 10, 100)
        spiro.generate()

        svg_path = os.path.join(tmpdir, "test.svg")
        spiro.to_svg(svg_path)

        assert os.path.exists(svg_path)
        assert os.path.getsize(svg_path) > 0


def test_export_stl():
    """Test STL export functionality"""
    with tempfile.TemporaryDirectory() as tmpdir:
        spiro = HorizontalSpirograph(40.0, 0.75, 0.6, 10, 100)
        spiro.generate()

        stl_path = os.path.join(tmpdir, "test.stl")
        spiro.to_stl(stl_path, depth=0.1)

        assert os.path.exists(stl_path)
        assert os.path.getsize(stl_path) > 0


def test_export_step():
    """Test STEP export functionality"""
    with tempfile.TemporaryDirectory() as tmpdir:
        spiro = HorizontalSpirograph(40.0, 0.75, 0.6, 10, 100)
        spiro.generate()

        step_path = os.path.join(tmpdir, "test.stp")
        spiro.to_step(step_path, depth=0.1)

        assert os.path.exists(step_path)
        assert os.path.getsize(step_path) > 0


def test_guilloche_export_all():
    """Test exporting all formats from GuillochePattern"""
    with tempfile.TemporaryDirectory() as tmpdir:
        pattern = GuillochePattern(radius=38.0)

        h_spiro = HorizontalSpirograph(38.0, 0.75, 0.6, 10, 100)
        pattern.add_layer(h_spiro)
        pattern.generate()

        base_path = os.path.join(tmpdir, "watch_face")
        pattern.export_all(base_path)

        assert os.path.exists(base_path + ".svg")
        assert os.path.exists(base_path + ".stl")
        assert os.path.exists(base_path + ".stp")


def test_flinque_layer():
    """Test FlinqueLayer creation and generation"""
    from turtles.turtles import FlinqueLayer

    flinque = FlinqueLayer(
        radius=38.0,
        num_petals=12,
        num_waves=40,
        wave_amplitude=2.0,
        wave_frequency=1.0,
        inner_radius_ratio=0.03,
    )
    assert flinque is not None


def test_watch_top_level():
    wf = WatchFace(radius=38.0)
    wf.add_inner()
    wf.add_outer()
    wf.add_center_hole()

    wf.add_flinque(
        radius=38.0,
        num_petals=12,  # 12 chevron peaks per ring (defines the petals)
        num_waves=40,  # 80 concentric rings for dense line work
        wave_amplitude=2.0,  # Chevron amplitude (how much the V points outward)
        wave_frequency=1.0,  # Fine ripple texture (1.0 = minimal ripple)
        inner_radius_ratio=0.03,  # Start very close to center
    )
    wf.generate()

    with tempfile.TemporaryDirectory() as tmpdir:
        svg_path = os.path.join(tmpdir, "guilloche_pattern.svg")
        wf.to_svg(svg_path)


def test_rose_engine_config():
    """Test RoseEngineConfig creation and presets"""
    from turtles import RoseEngineConfig

    # Test basic config
    config = RoseEngineConfig(base_radius=20.0, amplitude=2.0)
    assert config.base_radius == 20.0
    assert config.amplitude == 2.0
    assert config.resolution == 1000

    # Test preset configs
    config_huit = RoseEngineConfig.huit_eight(base_radius=20.0, amplitude=2.0)
    assert config_huit.base_radius == 20.0

    config_grain = RoseEngineConfig.grain_de_riz(base_radius=20.0, grain_size=1.0, amplitude=1.5)
    assert config_grain.base_radius == 20.0

    config_drap = RoseEngineConfig.draperie(base_radius=20.0, wave_frequency=6.0, amplitude=2.0)
    assert config_drap.base_radius == 20.0

    config_dia = RoseEngineConfig.diamant(base_radius=20.0, divisions=12, amplitude=1.5)
    assert config_dia.base_radius == 20.0


def test_rosette_pattern():
    """Test RosettePattern creation"""
    from turtles import RosettePattern

    # Test different pattern types
    circular = RosettePattern.circular()
    assert circular is not None

    sinusoidal = RosettePattern.sinusoidal(frequency=5.0)
    assert sinusoidal is not None

    multi_lobe = RosettePattern.multi_lobe(lobes=8)
    assert multi_lobe is not None

    huit_eight = RosettePattern.huit_eight(lobes=8)
    assert huit_eight is not None

    grain_de_riz = RosettePattern.grain_de_riz(grain_size=1.0, rows=12)
    assert grain_de_riz is not None

    draperie = RosettePattern.draperie(frequency=6.0, depth_frequency=12.0)
    assert draperie is not None

    diamant = RosettePattern.diamant(divisions=12)
    assert diamant is not None


def test_cutting_bit():
    """Test CuttingBit creation"""
    from turtles import CuttingBit

    # Test different bit types
    v_bit = CuttingBit.v_shaped(angle=30.0, width=0.5)
    assert v_bit.width == 0.5
    assert v_bit.depth > 0.0

    flat_bit = CuttingBit.flat(width=1.0, depth=0.5)
    assert flat_bit.width == 1.0
    assert flat_bit.depth == 0.5

    round_bit = CuttingBit.round(diameter=2.0)
    assert round_bit.width == 2.0
    assert round_bit.depth == 1.0


def test_rose_engine_lathe():
    """Test RoseEngineLathe creation and generation"""
    from turtles import CuttingBit, RoseEngineConfig, RoseEngineLathe

    config = RoseEngineConfig(base_radius=20.0, amplitude=2.0)
    bit = CuttingBit.v_shaped(angle=30.0, width=0.5)
    lathe = RoseEngineLathe(config, bit)

    assert lathe is not None

    # Generate the pattern
    lathe.generate()


def test_rose_engine_svg_export():
    """Test rose engine SVG export"""
    from turtles import CuttingBit, RoseEngineConfig, RoseEngineLathe

    with tempfile.TemporaryDirectory() as tmpdir:
        config = RoseEngineConfig.huit_eight(base_radius=20.0, amplitude=2.0)
        bit = CuttingBit.v_shaped(angle=30.0, width=0.5)
        lathe = RoseEngineLathe(config, bit)
        lathe.generate()

        svg_path = os.path.join(tmpdir, "rose_pattern.svg")
        lathe.to_svg(svg_path)

        assert os.path.exists(svg_path)
        assert os.path.getsize(svg_path) > 0


def test_rose_engine_patterns():
    """Test all rose engine pattern presets"""
    from turtles import CuttingBit, RoseEngineConfig, RoseEngineLathe

    bit = CuttingBit.v_shaped(angle=30.0, width=0.5)

    # Test each pattern preset
    patterns = [
        RoseEngineConfig.huit_eight(base_radius=20.0, amplitude=2.0),
        RoseEngineConfig.grain_de_riz(base_radius=20.0, grain_size=1.0, amplitude=1.5),
        RoseEngineConfig.draperie(base_radius=20.0, wave_frequency=6.0, amplitude=2.0),
        RoseEngineConfig.diamant(base_radius=20.0, divisions=12, amplitude=1.5),
    ]

    for config in patterns:
        lathe = RoseEngineLathe(config, bit)
        lathe.generate()
        assert lathe is not None


def test_rose_engine_lathe_run():
    """Test multi-pass rose engine lathe run"""
    import os
    import tempfile

    from turtles import CuttingBit, RoseEngineConfig, RoseEngineLatheRun, RosettePattern

    # Create a config
    config = RoseEngineConfig(base_radius=20.0, amplitude=2.0)
    config.set_rosette(RosettePattern.multi_lobe(12))

    # Create cutting bit
    bit = CuttingBit.v_shaped(angle=30.0, width=0.5)

    # Test creation
    run = RoseEngineLatheRun(config, bit, num_passes=8)
    assert run is not None
    assert run.num_passes == 8

    # Test generation
    run.generate()

    # Test SVG export
    with tempfile.NamedTemporaryFile(mode="w", suffix=".svg", delete=False) as f:
        svg_path = f.name

    try:
        run.to_svg(svg_path)
        assert os.path.exists(svg_path)
        assert os.path.getsize(svg_path) > 0
    finally:
        if os.path.exists(svg_path):
            os.unlink(svg_path)


def test_rose_engine_lathe_run_patterns():
    """Test multi-pass rose engine with different patterns"""
    from turtles import CuttingBit, RoseEngineConfig, RoseEngineLatheRun, RosettePattern

    bit = CuttingBit.v_shaped(angle=30.0, width=0.1)

    # Test different rosette patterns
    patterns = [
        RosettePattern.huit_eight(lobes=8),
        RosettePattern.grain_de_riz(grain_size=1.0, rows=12),
        RosettePattern.draperie(frequency=6.0, depth_frequency=3.0),
        RosettePattern.diamant(divisions=12),
    ]

    for pattern in patterns:
        config = RoseEngineConfig(base_radius=20.0, amplitude=2.0)
        config.set_rosette(pattern)

        run = RoseEngineLatheRun(config, bit, num_passes=12)
        run.generate()
        assert run.num_passes == 12


def test_limacon_layer():
    """Test LimaconLayer creation and generation"""
    from turtles import LimaconLayer

    # Create a limaçon layer
    layer = LimaconLayer(num_curves=24, base_radius=20.0, amplitude=20.0, resolution=360)
    assert layer is not None
    assert layer.num_curves == 24
    assert layer.base_radius == 20.0
    assert layer.amplitude == 20.0

    # Generate the pattern
    layer.generate()

    # Get the lines
    lines = layer.get_lines()
    assert len(lines) == 24
    assert len(lines[0]) == 361  # resolution + 1 for closed curve


def test_limacon_matches_rose_engine():
    """Test that LimaconLayer produces identical output to RoseEngineLatheRun with sinusoidal frequency=1"""
    from turtles import CuttingBit, LimaconLayer, RoseEngineConfig, RoseEngineLatheRun, RosettePattern

    # Parameters for comparison
    num_curves = 12
    base_radius = 20.0
    amplitude = 20.0
    resolution = 360

    # Create LimaconLayer
    limacon = LimaconLayer(
        num_curves=num_curves,
        base_radius=base_radius,
        amplitude=amplitude,
        resolution=resolution,
    )
    limacon.generate()

    # Create equivalent RoseEngineLatheRun with sinusoidal frequency=1
    config = RoseEngineConfig(base_radius=base_radius, amplitude=amplitude)
    config.set_rosette(RosettePattern.sinusoidal(frequency=1.0))
    config.set_resolution(resolution)

    bit = CuttingBit.v_shaped(angle=30.0, width=0.02)
    # segments_per_pass=1 means complete shapes without gaps
    rose_run = RoseEngineLatheRun(config, bit, num_passes=num_curves, segments_per_pass=1)
    rose_run.generate()

    # Get lines from both
    limacon_lines = limacon.get_lines()
    rose_lines = rose_run.get_lines()

    # Both should have the same number of curves
    assert len(limacon_lines) == len(rose_lines), f"LimaconLayer has {len(limacon_lines)} curves, RoseEngineLatheRun has {len(rose_lines)} curves"

    # Each curve should have the same number of points
    for i, (lim_curve, rose_curve) in enumerate(zip(limacon_lines, rose_lines)):
        assert len(lim_curve) == len(rose_curve), (
            f"Curve {i}: LimaconLayer has {len(lim_curve)} points, RoseEngineLatheRun has {len(rose_curve)} points"
        )

        # Compare all points - they should be identical (within floating point tolerance)
        for j, (lim_pt, rose_pt) in enumerate(zip(lim_curve, rose_curve)):
            dist = ((lim_pt[0] - rose_pt[0]) ** 2 + (lim_pt[1] - rose_pt[1]) ** 2) ** 0.5
            assert dist < 1e-10, f"Point {i},{j} differs: limacon=({lim_pt[0]}, {lim_pt[1]}), rose=({rose_pt[0]}, {rose_pt[1]}), dist={dist}"


def test_draperie_pattern_displacement():
    """Test that the draperie pattern can be created and generates output"""
    from turtles import CuttingBit, RoseEngineConfig, RoseEngineLathe

    # Create draperie config
    config = RoseEngineConfig.draperie(base_radius=20.0, wave_frequency=6.0, amplitude=2.0)
    bit = CuttingBit.v_shaped(angle=30.0, width=0.1)

    # Create lathe and generate
    lathe = RoseEngineLathe(config, bit)
    lathe.generate()

    # If we get here, the pattern was successfully generated
    assert lathe is not None


def test_draperie_svg_export():
    """Test creating a draperie pattern and exporting to SVG"""
    from turtles import CuttingBit, RoseEngineConfig, RoseEngineLatheRun

    # Create draperie config
    config = RoseEngineConfig.draperie(base_radius=20.0, wave_frequency=6.0, amplitude=2.0)
    bit = CuttingBit.v_shaped(angle=30.0, width=0.1)

    # Create multi-pass run
    run = RoseEngineLatheRun(config, bit, num_passes=12, segments_per_pass=1)
    run.generate()

    # Export to SVG
    with tempfile.TemporaryDirectory() as tmpdir:
        svg_path = os.path.join(tmpdir, "draperie_test.svg")
        run.to_svg(svg_path)

        assert os.path.exists(svg_path), "SVG file should exist"
        assert os.path.getsize(svg_path) > 0, "SVG file should have content"


def test_draperie_multi_pass_creates_wavey_circles():
    """Test that RoseEngineLatheRun with draperie creates multiple complete curves"""
    from turtles import CuttingBit, RoseEngineConfig, RoseEngineLatheRun

    # Create draperie config
    config = RoseEngineConfig.draperie(base_radius=20.0, wave_frequency=6.0, amplitude=2.0)
    bit = CuttingBit.v_shaped(angle=30.0, width=0.1)

    # Create multi-pass run with segments_per_pass=1 (complete curves)
    num_passes = 16
    run = RoseEngineLatheRun(config, bit, num_passes=num_passes, segments_per_pass=1)
    run.generate()

    # Get the generated lines
    lines = run.get_lines()

    # Should have one curve per pass
    assert len(lines) == num_passes, f"Expected {num_passes} curves, got {len(lines)}"

    # Each curve should have points (resolution + 1 for closed curve)
    for i, curve in enumerate(lines):
        assert len(curve) > 0, f"Curve {i} should have points"

    # Verify that different passes produce different curves (due to phase rotation)
    # Compare first and second curves - they should be different
    if len(lines) >= 2:
        curve1 = lines[0]
        curve2 = lines[1]

        # Calculate distance between corresponding points
        total_diff = 0.0
        for pt1, pt2 in zip(curve1[:10], curve2[:10]):  # Check first 10 points
            dist = ((pt1[0] - pt2[0]) ** 2 + (pt1[1] - pt2[1]) ** 2) ** 0.5
            total_diff += dist

        avg_diff = total_diff / 10.0
        assert avg_diff > 0.01, "Different passes should create different curves due to phase rotation"

