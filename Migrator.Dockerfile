FROM christophwurst/diesel-cli:latest

ADD migrations .

ENTRYPOINT ["diesel"]
CMD ["migration", "run"]

