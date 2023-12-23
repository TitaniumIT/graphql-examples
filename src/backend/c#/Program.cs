using GraphQL;
using GraphQLServer.helpers;
using GraphQL.Types.Relay;
using System.Text;
using GraphQLServer.Schema;
using GraphQL.Execution;
using GraphQL.MicrosoftDI;
using System.Net;

var builder = WebApplication.CreateBuilder(args);

// Add services to the container.

builder.Services.AddGraphQL(b => b
      .AddSystemTextJson()
      .AddSelfActivatingSchema<Domain>()
      .AddGraphTypes()
      .AddDataLoader()
      .AddAuthorizationRule()
      .AddUserContextBuilder( context =>
      {
          var managerSecret = context.Request.Headers.FirstOrDefault(h => h.Key.ToLower() == "managersecret").Value.FirstOrDefault();
          if (managerSecret == "I`m Manager")
          {
              context.User = new ManagerUser();
          }
          return new Dictionary<string, object?>();
      })
      .AddErrorInfoProvider(opt =>
      {
          opt.ExposeExceptionDetails = true;
      })
      .ConfigureExecutionOptions(options =>
      {
          options.ThrowOnUnhandledException = true;
      })
    //  .UseApolloTracing()
      );

builder.Services.AddLogging(builder => builder.AddConsole());
builder.Services.AddCors();
builder.Services.AddHttpContextAccessor();
builder.Services.AddAuthentication();
builder.Services.AddTransient(typeof(ConnectionType<>));
builder.Services.AddTransient(typeof(EdgeType<>));
builder.Services.AddTransient<PageInfoType>();
builder.Services.AddAuthorization(options =>
{
    options.AddPolicy("IsManager", policy =>
    {
        policy.RequireRole("Manager");
    });
});

var app = builder.Build();

// Configure the HTTP request pipeline.
app.UseRouting();
app.UseAuthentication();
app.UseAuthorization();
app.UseWebSockets();
app.UseCors(options =>
{
    options.AllowAnyMethod();
    options.AllowAnyOrigin();
    options.AllowAnyHeader();
});

app.UseDeveloperExceptionPage();

app.UseEndpoints(endpoints =>
{
    endpoints.MapGet("/", async (context) =>
    {
        context.Response.ContentType = "text/html";
        await context.Response.BodyWriter.WriteAsync(
            Encoding.UTF8.GetBytes("<a href='ui/playground'>Playground</a><br><a href='ui/voyager'>Voyager</a>"));
        await context.Response.CompleteAsync();
    });

    endpoints.MapGraphQL<Domain>("graphql/{operation=default}",configureMiddleware: options =>
    {
        options.HandleWebSockets = true;
        options.ExecuteBatchedRequestsInParallel = true;
    });
});
app.UseGraphQLPlayground();
app.UseGraphQLVoyager();

app.Run();