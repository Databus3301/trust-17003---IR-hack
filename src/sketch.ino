const int pwm1 = 5;
const int pwm2 = 3;

void setup() {
    pinMode(pwm1, OUTPUT);
    pinMode(pwm2, OUTPUT);
}

void left(int speed, int time) {
  digitalWrite(pwm2, LOW);
  analogWrite(pwm1, speed);
  delay(time);
}

void right(int speed, int time) {
  digitalWrite(pwm1, LOW);
  analogWrite(pwm2, speed);
  delay(time);
}

void loop() {
    left(255, 500);
    delay(100);
    right(255, 500);
    delay(100);
}

