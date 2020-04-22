FROM christophwurst/diesel-cli:latest

COPY migrations migrations

ENTRYPOINT ["diesel"]
CMD ["migration", "run"]

