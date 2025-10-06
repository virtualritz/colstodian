use colstodian::basic_encodings::*;
use colstodian::details::encodings::{EncodedSrgbF32, Oklab};
use colstodian::*;
use criterion::{Criterion, black_box, criterion_group, criterion_main};

fn bench_srgb_u8_to_linear(c: &mut Criterion) {
    let colors: Vec<Color<SrgbU8>> = (0..1000)
        .map(|i| {
            Color::srgb_u8(
                (i % 256) as u8,
                ((i * 17) % 256) as u8,
                ((i * 73) % 256) as u8,
            )
        })
        .collect();

    c.bench_function("srgb_u8_to_linear", |b| {
        b.iter(|| {
            colors
                .iter()
                .map(|color| black_box(color.convert::<LinearSrgb>()))
                .collect::<Vec<_>>()
        })
    });
}

fn bench_linear_to_srgb_u8(c: &mut Criterion) {
    let colors: Vec<Color<LinearSrgb>> = (0..1000)
        .map(|i| {
            let f = i as f32 / 1000.0;
            Color::linear_srgb(f, (f * 2.0) % 1.0, (f * 3.0) % 1.0)
        })
        .collect();

    c.bench_function("linear_to_srgb_u8", |b| {
        b.iter(|| {
            colors
                .iter()
                .map(|color| black_box(color.convert::<SrgbU8>()))
                .collect::<Vec<_>>()
        })
    });
}

fn bench_srgb_f32_to_linear(c: &mut Criterion) {
    let colors: Vec<Color<EncodedSrgbF32>> = (0..1000)
        .map(|i| {
            let f = i as f32 / 1000.0;
            Color::srgb_f32(f, (f * 2.0) % 1.0, (f * 3.0) % 1.0)
        })
        .collect();

    c.bench_function("srgb_f32_to_linear", |b| {
        b.iter(|| {
            colors
                .iter()
                .map(|color| black_box(color.convert::<LinearSrgb>()))
                .collect::<Vec<_>>()
        })
    });
}

fn bench_oklab_conversion(c: &mut Criterion) {
    let colors: Vec<Color<SrgbU8>> = (0..1000)
        .map(|i| {
            Color::srgb_u8(
                (i % 256) as u8,
                ((i * 17) % 256) as u8,
                ((i * 73) % 256) as u8,
            )
        })
        .collect();

    c.bench_function("srgb_u8_to_oklab", |b| {
        b.iter(|| {
            colors
                .iter()
                .map(|color| black_box(color.convert::<Oklab>()))
                .collect::<Vec<_>>()
        })
    });
}

fn bench_perceptual_blend(c: &mut Criterion) {
    let color1 = Color::srgb_u8(255, 100, 50).convert::<Oklab>();
    let color2 = Color::srgb_u8(50, 100, 255).convert::<Oklab>();

    c.bench_function("oklab_perceptual_blend", |b| {
        b.iter(|| {
            for t in [0.0, 0.25, 0.5, 0.75, 1.0] {
                black_box(color1.perceptual_blend(color2, t));
            }
        })
    });
}

fn bench_linear_math_operations(c: &mut Criterion) {
    let colors: Vec<Color<LinearSrgb>> = (0..1000)
        .map(|i| {
            let f = i as f32 / 1000.0;
            Color::linear_srgb(f, (f * 2.0) % 1.0, (f * 3.0) % 1.0)
        })
        .collect();

    c.bench_function("linear_color_addition", |b| {
        b.iter(|| {
            colors
                .chunks(2)
                .map(|chunk| black_box(chunk[0] + chunk[1]))
                .collect::<Vec<_>>()
        })
    });

    c.bench_function("linear_color_scalar_mul", |b| {
        b.iter(|| {
            colors
                .iter()
                .map(|color| black_box(*color * 0.5))
                .collect::<Vec<_>>()
        })
    });
}

criterion_group!(
    benches,
    bench_srgb_u8_to_linear,
    bench_linear_to_srgb_u8,
    bench_srgb_f32_to_linear,
    bench_oklab_conversion,
    bench_perceptual_blend,
    bench_linear_math_operations
);
criterion_main!(benches);
