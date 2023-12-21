namespace GraphQLServer.Models
{
    internal class Category
    {
        public string Id { get; set;  }
        public string Name { get; set;  }

        public Category(string id)
        {
            Id = id;
            Name = "ref-only";
        }

        public Category(string id, string name)
        {
            Id = id;
            Name = name;
        }
    }
}



