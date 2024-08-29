# CSE381 Week 11B Solution
# Image Processing - Convex Hull

import matplotlib.pyplot as plt
from skimage.morphology import convex_hull_image, convex_hull_object
from skimage import io, color
from skimage.util import invert
import numpy as np
from skimage.transform import resize
from scipy.spatial.distance import directed_hausdorff


def display(image, hull):
    fig, axes = plt.subplots(1, 2)
    ax = axes.ravel()

    ax[0].set_title('Original Picture')
    ax[0].imshow(image, cmap=plt.cm.gray)
    ax[0].set_axis_off()

    ax[1].set_title('Convex Hull')
    ax[1].imshow(hull, cmap=plt.cm.gray)
    ax[1].set_axis_off()

    plt.tight_layout()
    plt.show()



def compute_convex_hull(image_path):
    # Read the image and convert it to grayscale
    image = io.imread(image_path)
    gray_image = color.rgb2gray(image)

    # Need to invert (black <-> white) and 
    # standardize the size
    binary_image = invert(gray_image > 0.5)
    binary_image = resize(binary_image, (300,300), anti_aliasing=False)

    # Create the convex hull
    convex_hull = convex_hull_image(binary_image)
    
    return binary_image, convex_hull

def compare_hulls(hull1, hull2):
    # Convert convex hulls to sets of points
    points_hull1 = np.argwhere(hull1)
    points_hull2 = np.argwhere(hull2)

    # Compute the directed Hausdorff distance from hull1 to hull2
    hausdorff_distance_1_to_2 = directed_hausdorff(points_hull1, points_hull2)[0]

    # Compute the directed Hausdorff distance from hull2 to hull1
    hausdorff_distance_2_to_1 = directed_hausdorff(points_hull2, points_hull1)[0]

    # Compute the symmetric Hausdorff distance
    hausdorff_distance = max(hausdorff_distance_1_to_2, hausdorff_distance_2_to_1)

    return hausdorff_distance

# Convert image to grayscale and then invert (black->white, white->black)
# image = invert(color.rgb2gray(io.imread("mario.png")) > 0.5)
image = io.imread("mario.png")
image = color.rgb2gray(image)
image = invert(image > 0.5)
hull = convex_hull_image(image)
display(image, hull)
    
image = io.imread("stars.jpg")
image = color.rgb2gray(image)
image = invert(image > 0.5)
hull = convex_hull_object(image)
display(image, hull)


image, hull = compute_convex_hull("horse1.jpg")
image1, hull1 = compute_convex_hull("dino.jpg")
image2, hull2 = compute_convex_hull("key.jpg")
image3, hull3 = compute_convex_hull("horse2.jpg")
image4, hull4 = compute_convex_hull("tree.jpg")

display(image,hull)
display(image1,hull1)
display(image2,hull2)
display(image3,hull3)
display(image4,hull4)

# 0 - Same
# < 10 - Very similair
# 10 - 50 - Small Difference
# 50 - 100 - Medium Difference
# > 100 - Large Difference
print(compare_hulls(hull,hull))
print(compare_hulls(hull,hull1))
print(compare_hulls(hull,hull2))
print(compare_hulls(hull,hull3))
print(compare_hulls(hull,hull4))