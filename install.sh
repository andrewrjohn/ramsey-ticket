#!/bin/bash

echo ""
echo "Installing Ramsey Ticket..."
echo ""

mkdir ~/.RamseyTicket

curl "https://raw.githubusercontent.com/andrewrjohn/ramsey-ticket/master/target/release/ramsey-ticket" >> ~/.RamseyTicket

echo ""
echo "Finished!"

exit 0