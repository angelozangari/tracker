FROM postgres:15

ENV POSTGRES_DB=tracker
ENV POSTGRES_USER=postgres
ENV POSTGRES_PASSWORD=password

EXPOSE 5432