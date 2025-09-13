#!/usr/bin/env python3
import numpy as np
import struct

def create_yuv422p_test_image(width=256, height=256, filename="test_256x256.yuv422p"):
    """Create a test YUV 4:2:2 planar image with color bars pattern"""

    # Create Y plane (luma)
    y_plane = np.zeros((height, width), dtype=np.uint8)

    # Create color bars pattern
    bar_width = width // 8
    for i in range(8):
        x_start = i * bar_width
        x_end = (i + 1) * bar_width if i < 7 else width
        # Different brightness levels
        y_value = int(16 + (219 * i / 7))  # ITU-R BT.601 range
        y_plane[:, x_start:x_end] = y_value

    # Create U and V planes (chroma) - 4:2:2 means half horizontal resolution
    u_plane = np.zeros((height, width // 2), dtype=np.uint8)
    v_plane = np.zeros((height, width // 2), dtype=np.uint8)

    # Color bars chroma values
    u_values = [128, 90, 54, 240, 165, 128, 90, 128]  # Approximate color bar U values
    v_values = [128, 240, 110, 110, 165, 128, 240, 128]  # Approximate color bar V values

    bar_width_chroma = width // 16  # Half resolution
    for i in range(8):
        x_start = i * bar_width_chroma
        x_end = (i + 1) * bar_width_chroma if i < 7 else width // 2
        u_plane[:, x_start:x_end] = u_values[i]
        v_plane[:, x_start:x_end] = v_values[i]

    # Write to file in planar format (all Y, then all U, then all V)
    with open(filename, 'wb') as f:
        f.write(y_plane.tobytes())
        f.write(u_plane.tobytes())
        f.write(v_plane.tobytes())

    print(f"Created {filename}")
    print(f"  Dimensions: {width}x{height}")
    print(f"  Format: YUV 4:2:2 planar")
    print(f"  Y plane size: {width * height} bytes")
    print(f"  U plane size: {width * height // 2} bytes")
    print(f"  V plane size: {width * height // 2} bytes")
    print(f"  Total size: {width * height + width * height} bytes")

    return filename

def create_ppm_test_image(width=256, height=256, filename="test_256x256.ppm"):
    """Create a test PPM image with gradient pattern"""

    with open(filename, 'w') as f:
        # PPM header
        f.write(f"P3\n{width} {height}\n255\n")

        # Create gradient pattern
        for y in range(height):
            for x in range(width):
                r = int(255 * x / width)
                g = int(255 * y / height)
                b = int(255 * (x + y) / (width + height))
                f.write(f"{r} {g} {b}\n")

    print(f"Created {filename}")
    print(f"  Dimensions: {width}x{height}")
    print(f"  Format: PPM (P3, ASCII)")

    return filename

if __name__ == "__main__":
    # Create test images
    yuv_file = create_yuv422p_test_image(256, 256, "test-data/test_256x256.yuv422p")
    ppm_file = create_ppm_test_image(256, 256, "test-data/test_256x256.ppm")

    # Also create smaller test images for quick testing
    create_yuv422p_test_image(64, 64, "test-data/test_64x64.yuv422p")
    create_ppm_test_image(64, 64, "test-data/test_64x64.ppm")
