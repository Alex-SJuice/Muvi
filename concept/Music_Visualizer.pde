import processing.sound.*;
PVector convert (PVector cord)
{
  PVector g = new PVector(cord.x, cord.y);
  g.y *= -1;
  
  //x dimension of scene
  g.x += (204 / 2);
  
  //y dimension of scene
  g.y += (100 / 2);
  
  return g;
}
PVector pv (float a, float b)
{
  PVector out = new PVector (a,b,0);
  return out;
}
float calcDis (PVector a, PVector b)
{
  float x = a.x - b.x;
  float y = a.y - b.y;
  return sqrt(abs(x)*abs(x)+abs(y)*abs(y));
}


SoundFile song;
FFT fft;
Amplitude amp;
int bands = 512;
color from = color(0,0,0);
color linecol = color(0,0,0);
float ang = 0;
float radius;

float[] current = new float[bands];
float[] back1 = new float[bands];
float[] back2 = new float[bands];
float[] back3 = new float[bands];
float[] back4 = new float[bands];

void setup()
{
  size(1000,500);
  windowRatio(204, 100);
  // Create a new sample object.
  song = new SoundFile(this, "song.mp3");
  
  song.play();
  from = #FFFFFF;
  linecol = #EA6D6D;
  amp = new Amplitude(this);
  fft = new FFT(this, bands);
  amp.input(song);
  fft.input(song);
}

void draw() {
  background(from);
  fft.analyze();
  radius = map(pow(amp.analyze(),2),0,1,25,28);
  
  for (int i = 0; i < bands; i++) {
    back4[i] = back3[i];
    back3[i] = back2[i];
    back2[i] = back1[i];
    back1[i] = current[i];
    current[i] = map(fft.spectrum[i], 0, 1, 0, 200);
  }
  
  stroke(linecol);  
  for (int i = 0; i < 128; i++) {
    float y = sqrt(
      (current[i] * 0.5 +
      current[i == 0 ? 127 : i-1] * 0.25 +
      current[i == 127 ? 0 : i+1] * 0.25) * 0.35 +
      back1[i] * 0.25 + 
      back2[i] * 0.15 + 
      back3[i] * 0.15 + 
      back4[i] * 0.1) * 3;
    
    PVector p1 = convert(pv(
      radius * cos(2*PI/128 * i + ang),
      radius * sin(2*PI/128 * i + ang)));
    PVector p2 = convert(pv(
      (radius + y) * cos(2*PI/128 * i + ang),
      (radius + y) * sin(2*PI/128 * i + ang)));
    
    line(p1.x,p1.y,p2.x,p2.y);
  }

  ang -= 0.003;
}
