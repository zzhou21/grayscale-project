### The wasm file is under host/build/grayscale_project.wasm

This project processes hexadecimal image data and converts it into a grayscale format using Rust.

How It Works:
Hexadecimal Image Processing (process_hex_image)

This function takes RGB pixel data and converts it into a grayscale image.
It applies the luminance formula:
gray
=
(
𝑅
×
0.299
)
+
(
𝐺
×
0.587
)
+
(
𝐵
×
0.114
)
gray=(R×0.299)+(G×0.587)+(B×0.114)

It iterates through the RGB data in chunks of 3 bytes (R, G, B) and computes the grayscale intensity.
The resulting grayscale values are stored in a Vec<u8>.
Default Image Size

The grayscale image is assumed to have a fixed width and height of 100x100 pixels.

# Project: Grayscale Image Processing from Hex Input

This project processes **hexadecimal image data** and converts it into a **grayscale format** using Rust.

## How It Works:

### 1. Hexadecimal Image Processing (`process_hex_image`)

- The function `process_hex_image` takes a **hexadecimal string (as a pointer)** and its **length**.
- It **validates the input**, extracts the hex data, and converts it into a `Vec<u8>` byte array.
- The processed data is then passed to the **grayscale conversion function**.

### 2. RGB to Grayscale Conversion (`rgb_to_grayscale`)

- This function takes **RGB pixel data** and converts it into a **grayscale image**.

- It applies the **luminance formula**:

- It iterates through the **RGB data in chunks of 3 bytes (R, G, B)** and computes the grayscale intensity.

- The resulting grayscale values are stored in a `Vec<u8>`.

### 3. Default Image Size

- The grayscale image is assumed to have a fixed **width and height of 100x100 pixels**.

---

## Test Case

### Input (Fully Red Image - 3x3 Example)

A **fully red image** means every pixel in the image has the RGB values:

Each pixel will be converted to grayscale using the formula:

Substituting the values:

#### **Input (RGB Values):**

```plaintext
[
  255, 0, 0,   255, 0, 0,   255, 0, 0,
  255, 0, 0,   255, 0, 0,   255, 0, 0,
  255, 0, 0,   255, 0, 0,   255, 0, 0
]
```

#### **Expected Output (Grayscale Values):**

```plaintext
[
  76, 76, 76,
  76, 76, 76,
  76, 76, 76
]
```

This confirms that a fully red image is converted to a **dark gray shade with intensity 76 for all pixels**.
