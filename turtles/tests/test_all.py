import os
import tempfile

from turtles import GuillochePattern, HorizontalSpirograph, SphericalSpirograph, VerticalSpirograph


def test_all():
    assert True


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
