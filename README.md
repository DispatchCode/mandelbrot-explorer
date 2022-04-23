# Mandelbrot Fractal explorer

Inspired by my own [fraCtal](https://github.com/DispatchCode/fraCtal) project; code & formulas explanations are there too.
Information about the mandelbrot set can also be found on [Wikipedia](https://en.wikipedia.org/wiki/Mandelbrot_set).

For better performance build & run with release flag 😎:

```
cargo run --release
```
The code produced by Rust heavily makes use of the MMX set, as you can see down below (`mandelbrot_fractal()` function):

![2022-04-24_011503](https://user-images.githubusercontent.com/4256708/164949367-02cce677-968e-4012-b3b0-ad4f4360bd83.png)

### Features:
✅ Mandelbrot Fractal <br>
✅ SDL2 (you need SDL2 to run the project) <br>
✅ save the generated fractal as image (PNG) <br>
✅ size, zoom, colors can bhe changed <br>
❌ ImGui support (or a GUI menu where options can be changed) <br>
❌ multithreading support

### Future plans:
🎯 ImGui support <br>

---
## Generated images

```Rust
let (val_in, val_out) = (32, 0);
let g = if i >= 32 {
    (255.0
        - (((i + val_in) as f32 / (max_iterations + val_in) as f32)
            * (255 - val_out) as f32)) as u8
} else {
    0 as u8
};
```

![mandelbrot0](https://user-images.githubusercontent.com/4256708/164943526-16890613-b342-43c4-9453-102be268403b.png)

---

```Rust
let attenuation = false; // disable attenuation
// ...
let (val_in, val_out) = (32, 150);
let g = if i >= 32 {
    (255.0
        - (((i + val_in) as f32 / (max_iterations + val_in) as f32)
            * (255 - val_out) as f32)) as u8
} else {
    0 as u8
};
```
![mandelbrot](https://user-images.githubusercontent.com/4256708/164946372-5da42f27-840b-4196-8d3f-8d4e501fcd8c.png)

---

```Rust
let attenuation = false;

// ...

let (val_in, val_out) = (32, 0);
let g = if i >= 32 {
    (255.0
        - (((i + val_in) as f32 / (max_iterations + val_in) as f32)
            * (255 - val_out) as f32)) as u8
} else {
    0 as u8
};
```
![mandelbrot](https://user-images.githubusercontent.com/4256708/164946435-5a032e9c-258c-49fe-9269-cf138484aeec.png)

---

```Rust
let attenuation = false;

// ...

let (val_in, val_out) = (32, 0);
let g = if i >= 0 {
    (255.0
        - (((i + val_in) as f32 / (max_iterations + val_in) as f32)
            * (255 - val_out) as f32)) as u8
} else {
    0 as u8
};
```

![mandelbrot](https://user-images.githubusercontent.com/4256708/164946477-f2c95510-e42e-4755-8caa-b10c4554b613.png)

---
```Rust
let attenuation = true;

let (blu_a, blu_b) = (255.0, 255.0);
let (red_a, red_b) = (0.0, 255.0);

let (val_in, val_out) = (128, 0);
let g = if i >= 32 {
    (255.0
        - (((i + val_in) as f32 / (max_iterations + val_in) as f32)
            * (255 - val_out) as f32)) as u8
} else {
    0 as u8
};
```

![mandelbrot](https://user-images.githubusercontent.com/4256708/164946619-e0fe9141-23ee-4b8c-bf0c-7d3e85248396.png)

---
```Rust
let attenuation = true;
let (x_min, x_max, y_min, y_max) = (0.40, 0.37, 0.26, 0.21);

// The colors around the lake
let (blu_a, blu_b) = (32.0, 255.0);
let (red_a, red_b) = (32.0, 255.0);

let (val_in, val_out) = (180,0);
let g = if i >= 128 {
    (255.0
        - (((i + val_in) as f32 / (max_iterations + val_in) as f32)
          * (255 - val_out) as f32)) as u8
    } else {
        32 as u8
};
```

![mandelbrot](https://user-images.githubusercontent.com/4256708/164948369-3117a0e7-f6c3-4789-8064-c29917ec3b3c.png)