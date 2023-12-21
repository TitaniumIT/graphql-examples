using Microsoft.Extensions.Logging;
using System.Reactive.Subjects;

namespace GraphQLServer.Models
{
    internal class StaticDataSource
    {
        static List<Category> _categories = new(){
            new ("1" , "metals"),
            new ("2", "wood" ),
            new ("3", "non-food" ),
            new ("4", "food" )
        };

        // Slow backend call
        static public List<Category> Categories
        {
            get
            {
                Thread.Sleep(1000);
                return _categories;
            }
        }

        static public List<Product> Products { get; } = new()
        {
            new ("1" , "titanium" , "Strong metal" , new[] { "1","3" },10),
            new ("2" , "oak" , "strong wood" , new[] {"2", "3" }, 5),
            new ("3" , "iron" , "a metal" , new[] {"1", "3" }, 5),
            new ("4" , "silver" , "a precious metal" , new[] {"1", "3" }, 5),
            new ("5" , "gold" , "a rare precious metal" , new[] {"1", "3" }, 5),
            new ("6" , "porc" , "meat based on a Pig" , new[] {"4", }, 5),
            new ("7" , "beef" , "meat base on a cow" , new[] {"4" }, 5),
            new ("8" , "bread" , "made from wheat" , new[] {"4" }, 5)
        };

        static public List<ProductInBackorder> ProductInBackorders { get; } = new();
        static public List<ProductInTransit> ProductInTransit { get; } = new();

        static public Subject<ProductInTransit> ProductInTransitEvents = new();
    }
}



