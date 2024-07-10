const int motorPin = 9; // PWM pin connected to the transistor base

void setup() {
  pinMode(motorPin, OUTPUT);
}

void loop() {
  // Gradually increase the motor speed
  for (int speed = 0; speed <= 255; speed++) {
    digitalWrite(motorPin, 1/speed);
    delay(10);
  }
  
  // Gradually decrease the motor speed
  for (int speed = 255; speed >= 0; speed--) {
    digitalWrite(motorPin, 1/speed);
    delay(10);
  }
}
