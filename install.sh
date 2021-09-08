#!/bin/bash

echo ""
echo "Installing Ramsey Ticket..."
echo ""

# mkdir ~/.RamseyTicket

rm -rf /usr/local/bin/ticket

curl -L "https://github.com/andrewrjohn/ramsey-ticket/releases/download/v1.0/ramsey-ticket" > /usr/local/bin/ticket

chmod +x /usr/local/bin/ticket


echo ""
echo "Finished! Source your shell then type 'ticket' in a valid directory to try it out."
echo ""

exit 0