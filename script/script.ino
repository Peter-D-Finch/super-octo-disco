int analogPin = A0; // potentiometer wiper (middle terminal) connected to analog pin 3
                    // outside leads to ground and +5V
int val0 = 0;  // variable to store the value read
int val1 = 0;  // variable to store the value read
int val2 = 0;  // variable to store the value read

void setup() {
  Serial.begin(9600);           //  setup serial
  Serial.println("Hello world!");
  pinMode(LED_BUILTIN, OUTPUT);
}

void loop() {
  val0 = analogRead(A0);  // read the input pin
  val1 = analogRead(A1);  // read the input pin
  val2 = analogRead(A2);  // read the input pin
  Serial.println(val0);          // debug value
  //Serial.println(val1);          // debug value
  //Serial.println(val2);          // debug value
  digitalWrite(LED_BUILTIN, HIGH);   // turn the LED on (HIGH is the voltage level)
  delay(50);
  digitalWrite(LED_BUILTIN, LOW);    // turn the LED off by making the voltage LOW
}
