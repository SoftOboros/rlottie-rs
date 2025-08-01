// Copyright © SoftOboros Technology, Inc.
// SPDX-License-Identifier: MIT
#include <rlottie.h>
#define STB_IMAGE_WRITE_IMPLEMENTATION
#include "stb_image_write.h"
#include <iostream>
#include <memory>
#include <string>

int main(int argc, char **argv) {
    if (argc < 5) {
        std::cerr << "Usage: lottie2png <json> <width> <height> <frame>\n";
        return 1;
    }
    std::string file = argv[1];
    int w = std::atoi(argv[2]);
    int h = std::atoi(argv[3]);
    int frame = std::atoi(argv[4]);

    auto animation = rlottie::Animation::loadFromFile(file);
    if (!animation) {
        std::cerr << "Failed to load " << file << "\n";
        return 1;
    }
    std::unique_ptr<uint32_t[]> buffer(new uint32_t[w * h]);
    rlottie::Surface surface(buffer.get(), w, h, w * 4);
    animation->renderSync(frame, surface);

    // convert ARGB to RGBA
    for (int i = 0; i < w * h; ++i) {
        uint32_t argb = buffer[i];
        uint8_t a = (argb >> 24) & 0xFF;
        uint8_t r = (argb >> 16) & 0xFF;
        uint8_t g = (argb >> 8) & 0xFF;
        uint8_t b = argb & 0xFF;
        buffer[i] = (r << 24) | (g << 16) | (b << 8) | a;
    }

    std::string out = file + "_" + std::to_string(frame) + ".png";
    if (!stbi_write_png(out.c_str(), w, h, 4, buffer.get(), w * 4)) {
        std::cerr << "Failed to write PNG" << std::endl;
        return 1;
    }
    std::cout << out << std::endl;
    return 0;
}
