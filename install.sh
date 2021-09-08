#!/bin/bash

echo ""
echo "Installing Ramsey Ticket..."
echo ""

mkdir ~/.RamseyTicket

curl "https://github.com/andrewrjohn/ramsey-ticket/releases/download/v1.0/ramsey-ticket" >> ~/.RamseyTicket

echo ""
echo "Finished!"

exit 0