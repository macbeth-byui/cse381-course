import random
import pygame

class Point:
    def __init__(self, x, y, radius, root_qt):
        self.x = x
        self.y = y
        self.radius = radius
        self.qt = None  # Member of this quad tree to
        self.root_qt = root_qt # To allow for re-insertion
        self.dx = random.randint(1,10)*0.1
        self.dy = random.randint(1,10)*0.1
        self.color = (255,255,0)

    def move(self):
        self.x += self.dx
        self.y += self.dy
        if self.x > 1000:
            self.x = 0
        if self.x < 0:
            self.x = 1000
        if self.y > 1000:
            self.y = 0
        if self.y < 0:
            self.y = 1000
        self.qt.remove(self)
        self.root_qt.insert(self)

class QuadTree:

    MAX_LEVEL = 5
    MAX_OBJECTS = 5
    
    def __init__(self, level, x, y, height, width):
        self.x = x
        self.y = y
        self.height = height
        self.width = width
        self.level = level
        self.children = None
        self.objects = []
    
    def remove(self, point):
        self.objects.remove(point)
    
    def insert(self, point):
        midx = self.width // 2
        midy = self.height // 2
        if self.children is None:
            if len(self.objects) == QuadTree.MAX_OBJECTS:
                if self.level == QuadTree.MAX_LEVEL:
                    point.qt = self
                    self.objects.append(point)
                else:
                    self.objects.append(point)
                    self.children = []
                    self.children.append(QuadTree(self.level+1,self.x,self.y,midx,midy))
                    self.children.append(QuadTree(self.level+1,self.x+midx,self.y,midx,midy))
                    self.children.append(QuadTree(self.level+1,self.x, self.y+midy, midx, midy))
                    self.children.append(QuadTree(self.level+1,self.x+midx, self.y+midy, midx, midy))
                    for obj in self.objects:
                        self.insert(obj)
                    self.objects = []
            else:
                point.qt = self
                self.objects.append(point)
        else:
            if point.x < self.x+midx:
                left = True
            else:
                left = False
            if point.y < self.y+midy:
                top = True
            else:
                top = False
            if left and top:
                self.children[0].insert(point)
            elif not left and top:
                self.children[1].insert(point)
            elif left and not top:
                self.children[2].insert(point)
            else:
                self.children[3].insert(point)

qt = QuadTree(0,0,0,1000,1000)
points = []
for i in range(10000):
    x = Point(random.randint(1,999),random.randint(1,999),1,qt)
    points.append(x)
    qt.insert(x)


pygame.init()
win = pygame.display.set_mode((1000,1000))
clock = pygame.time.Clock()
running = True
while running:
    for event in pygame.event.get():
        if event.type == pygame.QUIT:
            running = False

    win.fill((0,0,0))
    for point in points:
        point.move()
    for point1 in points:
        for point2 in point1.qt.objects:
            if point1 != point2:
                if (abs(point1.x - point2.x) < 2) and (abs(point1.y - point2.y) < 2):
                    point1.dx *= -1
                    point1.dy *= -1
                    point1.color = (0,255,255)
                    point2.color = (0,255,255)
    for point in points:
        pygame.draw.circle(win,point.color,(point.x,point.y),point.radius)
    pygame.display.flip()
    clock.tick(30)

