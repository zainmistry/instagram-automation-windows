#!/usr/bin/env python3
from PIL import Image, ImageDraw, ImageFont
import os

# Create a 256x256 icon with Instagram-like gradient
size = 256
icon = Image.new('RGBA', (size, size), (0, 0, 0, 0))
draw = ImageDraw.Draw(icon)

# Create gradient background (Instagram-like)
for i in range(size):
    # Calculate gradient colors
    ratio = i / size
    r = int(225 + (150 - 225) * ratio)  # From pink to orange
    g = int(48 + (47 - 48) * ratio)
    b = int(108 + (224 - 108) * ratio)
    
    # Draw horizontal line
    draw.rectangle([(0, i), (size, i+1)], fill=(r, g, b, 255))

# Add rounded corners
mask = Image.new('L', (size, size), 0)
mask_draw = ImageDraw.Draw(mask)
corner_radius = 50
mask_draw.rounded_rectangle([(0, 0), (size, size)], corner_radius, fill=255)
icon.putalpha(mask)

# Add text or symbol (robot icon to indicate automation)
try:
    # Try to use a nice font if available
    font = ImageFont.truetype("/System/Library/Fonts/Helvetica.ttc", 120)
except:
    font = None

# Draw a simple robot icon using shapes
robot_color = (255, 255, 255, 255)
# Head
draw.rounded_rectangle([(80, 60), (176, 120)], 10, fill=robot_color)
# Eyes
draw.ellipse([(95, 75), (115, 95)], fill=(50, 50, 50, 255))
draw.ellipse([(141, 75), (161, 95)], fill=(50, 50, 50, 255))
# Body
draw.rounded_rectangle([(70, 130), (186, 190)], 10, fill=robot_color)
# Arms
draw.rounded_rectangle([(50, 140), (70, 180)], 5, fill=robot_color)
draw.rounded_rectangle([(186, 140), (206, 180)], 5, fill=robot_color)

# Create different sizes
sizes = [16, 32, 64, 128, 256]
os.makedirs('browser-core/icons', exist_ok=True)

for s in sizes:
    resized = icon.resize((s, s), Image.Resampling.LANCZOS)
    resized.save(f'browser-core/icons/icon_{s}x{s}.png')

# Save the main icon
icon.save('browser-core/icons/icon.png')

# Create ICO file for Windows with multiple sizes
icon.save('browser-core/icons/icon.ico', format='ICO', sizes=[(16, 16), (32, 32), (48, 48), (64, 64), (128, 128), (256, 256)])

print("✅ Icons created successfully!") 