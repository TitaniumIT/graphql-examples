FROM  mcr.microsoft.com/dotnet/aspnet:8.0 as base
LABEL org.opencontainers.image.authors="Titanium IT 2023"
WORKDIR /app
EXPOSE 8080

FROM mcr.microsoft.com/dotnet/sdk:8.0 AS build
WORKDIR /src
COPY . .
RUN dotnet restore 
RUN dotnet build
RUN dotnet test
RUN dotnet publish -o /app/publish /p:UseAppHost=false

FROM base AS final
WORKDIR /app
COPY --from=build /app/publish .
ENTRYPOINT ["dotnet", "GraphQLServer.dll"]
