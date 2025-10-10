Universal asnchronous receiver-transmitter

- peripheral device
- has start and end bits
- eariliest computer communication devices.
- UART is usually implemented in a IC and used for serial communication over computer or peripheral device serial port
- protocol for exchanging the serial data between two device
- serial data refers to the transmission of data one bit at a time over a single communication line or channel.
- simple, halfuplex and fullduplex 
- data is transmited as frames
- UART was one of teh earilies serial port COM ports
- SPI and I2C are replacing 
- ehternet and USB are used
- still important for lower speed low throutput applications
- the transmitter and reciever do not share a common clock
- transmitter and reciver threfrore must transmit at the same known speed (the reciver and transmitter end should have the same bud rate)


UART frame format:
- start/stop bits
- data bits
- parity bits (optional)

16550 UART is an integrated ciruit desinged for implemeting the interface for serial communications. The main features of 16500:

- ability to convert data from serial to parallel and from paralle to serial using shift register
- 16 bytes (transmit) + 16 bytes (receive)

