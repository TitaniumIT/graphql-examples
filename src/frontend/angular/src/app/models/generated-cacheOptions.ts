
      export interface PossibleTypesResultData {
        possibleTypes: {
          [key: string]: string[]
        }
      }
      const result: PossibleTypesResultData = {
  "possibleTypes": {
    "AllProductTypes": [
      "Product",
      "ProductInBackorder",
      "ProductInTransit"
    ],
    "AvailableActionsInterfaceType": [
      "Product",
      "ProductInBackorder",
      "ProductInTransit"
    ],
    "IProduct": [
      "Product",
      "ProductInBackorder",
      "ProductInTransit"
    ]
  }
};
      export default result;
    