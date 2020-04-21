FROM christophwurst/diesel-cli:latest

COPY migrations .

ENTRYPOINT ["diesel"]
CMD ["migration", "run"]

