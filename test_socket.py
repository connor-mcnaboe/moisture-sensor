import socket

def main():
    # Server details
    host = '192.168.140.216'
    port = 8080

    # Create a socket object
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        # Connect to the server
        s.connect((host, port))
        print("Connected to server.")

        # Keep receiving data from the server and print it
        try:
            while True:
                # Receive data from the server, buffer size 1024 bytes
                data = s.recv(1024)
                if not data:
                    break  # Stop if no data is received (server closed connection)
                print("Received:", data.decode('utf-8'))
        except KeyboardInterrupt:
            print("Disconnected from server.")

if __name__ == "__main__":
    main()
