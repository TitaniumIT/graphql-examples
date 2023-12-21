import { FieldPolicy, FieldReadFunction, TypePolicies, TypePolicy } from '@apollo/client/cache';
export type AvailableActionsInterfaceTypeKeySpecifier = ('actionsAllowed' | AvailableActionsInterfaceTypeKeySpecifier)[];
export type AvailableActionsInterfaceTypeFieldPolicy = {
	actionsAllowed?: FieldPolicy<any> | FieldReadFunction<any>
};
export type CategoryKeySpecifier = ('id' | 'name' | CategoryKeySpecifier)[];
export type CategoryFieldPolicy = {
	id?: FieldPolicy<any> | FieldReadFunction<any>,
	name?: FieldPolicy<any> | FieldReadFunction<any>
};
export type IProductKeySpecifier = ('description' | 'id' | 'name' | IProductKeySpecifier)[];
export type IProductFieldPolicy = {
	description?: FieldPolicy<any> | FieldReadFunction<any>,
	id?: FieldPolicy<any> | FieldReadFunction<any>,
	name?: FieldPolicy<any> | FieldReadFunction<any>
};
export type MutationTypeKeySpecifier = ('buy' | 'cancel' | 'deliver' | 'restock' | MutationTypeKeySpecifier)[];
export type MutationTypeFieldPolicy = {
	buy?: FieldPolicy<any> | FieldReadFunction<any>,
	cancel?: FieldPolicy<any> | FieldReadFunction<any>,
	deliver?: FieldPolicy<any> | FieldReadFunction<any>,
	restock?: FieldPolicy<any> | FieldReadFunction<any>
};
export type PageInfoKeySpecifier = ('endCursor' | 'hasNextPage' | 'hasPreviousPage' | 'startCursor' | PageInfoKeySpecifier)[];
export type PageInfoFieldPolicy = {
	endCursor?: FieldPolicy<any> | FieldReadFunction<any>,
	hasNextPage?: FieldPolicy<any> | FieldReadFunction<any>,
	hasPreviousPage?: FieldPolicy<any> | FieldReadFunction<any>,
	startCursor?: FieldPolicy<any> | FieldReadFunction<any>
};
export type ProductKeySpecifier = ('actionsAllowed' | 'categories' | 'categoriesWithoutBatchAsync' | 'categoriesWithoutBatchSync' | 'description' | 'id' | 'inBasket' | 'inStock' | 'isAllowedToBuy' | 'name' | 'nrCancelled' | 'nrDeliverd' | 'nrInTransit' | 'nrOrderd' | 'productsInTransit' | ProductKeySpecifier)[];
export type ProductFieldPolicy = {
	actionsAllowed?: FieldPolicy<any> | FieldReadFunction<any>,
	categories?: FieldPolicy<any> | FieldReadFunction<any>,
	categoriesWithoutBatchAsync?: FieldPolicy<any> | FieldReadFunction<any>,
	categoriesWithoutBatchSync?: FieldPolicy<any> | FieldReadFunction<any>,
	description?: FieldPolicy<any> | FieldReadFunction<any>,
	id?: FieldPolicy<any> | FieldReadFunction<any>,
	inBasket?: FieldPolicy<any> | FieldReadFunction<any>,
	inStock?: FieldPolicy<any> | FieldReadFunction<any>,
	isAllowedToBuy?: FieldPolicy<any> | FieldReadFunction<any>,
	name?: FieldPolicy<any> | FieldReadFunction<any>,
	nrCancelled?: FieldPolicy<any> | FieldReadFunction<any>,
	nrDeliverd?: FieldPolicy<any> | FieldReadFunction<any>,
	nrInTransit?: FieldPolicy<any> | FieldReadFunction<any>,
	nrOrderd?: FieldPolicy<any> | FieldReadFunction<any>,
	productsInTransit?: FieldPolicy<any> | FieldReadFunction<any>
};
export type ProductConnectionKeySpecifier = ('edges' | 'items' | 'pageInfo' | 'totalCount' | ProductConnectionKeySpecifier)[];
export type ProductConnectionFieldPolicy = {
	edges?: FieldPolicy<any> | FieldReadFunction<any>,
	items?: FieldPolicy<any> | FieldReadFunction<any>,
	pageInfo?: FieldPolicy<any> | FieldReadFunction<any>,
	totalCount?: FieldPolicy<any> | FieldReadFunction<any>
};
export type ProductContextTypeKeySpecifier = ('productGetV2' | ProductContextTypeKeySpecifier)[];
export type ProductContextTypeFieldPolicy = {
	productGetV2?: FieldPolicy<any> | FieldReadFunction<any>
};
export type ProductEdgeKeySpecifier = ('cursor' | 'node' | ProductEdgeKeySpecifier)[];
export type ProductEdgeFieldPolicy = {
	cursor?: FieldPolicy<any> | FieldReadFunction<any>,
	node?: FieldPolicy<any> | FieldReadFunction<any>
};
export type ProductInBackorderKeySpecifier = ('actionsAllowed' | 'description' | 'id' | 'name' | 'productId' | ProductInBackorderKeySpecifier)[];
export type ProductInBackorderFieldPolicy = {
	actionsAllowed?: FieldPolicy<any> | FieldReadFunction<any>,
	description?: FieldPolicy<any> | FieldReadFunction<any>,
	id?: FieldPolicy<any> | FieldReadFunction<any>,
	name?: FieldPolicy<any> | FieldReadFunction<any>,
	productId?: FieldPolicy<any> | FieldReadFunction<any>
};
export type ProductInTransitKeySpecifier = ('actionsAllowed' | 'customerId' | 'description' | 'id' | 'name' | 'productId' | 'state' | ProductInTransitKeySpecifier)[];
export type ProductInTransitFieldPolicy = {
	actionsAllowed?: FieldPolicy<any> | FieldReadFunction<any>,
	customerId?: FieldPolicy<any> | FieldReadFunction<any>,
	description?: FieldPolicy<any> | FieldReadFunction<any>,
	id?: FieldPolicy<any> | FieldReadFunction<any>,
	name?: FieldPolicy<any> | FieldReadFunction<any>,
	productId?: FieldPolicy<any> | FieldReadFunction<any>,
	state?: FieldPolicy<any> | FieldReadFunction<any>
};
export type QueryTypeKeySpecifier = ('allProducts' | 'categories' | 'product' | 'productContext' | 'products' | 'productsRelay' | 'productsSkipTake' | QueryTypeKeySpecifier)[];
export type QueryTypeFieldPolicy = {
	allProducts?: FieldPolicy<any> | FieldReadFunction<any>,
	categories?: FieldPolicy<any> | FieldReadFunction<any>,
	product?: FieldPolicy<any> | FieldReadFunction<any>,
	productContext?: FieldPolicy<any> | FieldReadFunction<any>,
	products?: FieldPolicy<any> | FieldReadFunction<any>,
	productsRelay?: FieldPolicy<any> | FieldReadFunction<any>,
	productsSkipTake?: FieldPolicy<any> | FieldReadFunction<any>
};
export type SubscriptionTypeKeySpecifier = ('statusChanged' | SubscriptionTypeKeySpecifier)[];
export type SubscriptionTypeFieldPolicy = {
	statusChanged?: FieldPolicy<any> | FieldReadFunction<any>
};
export type StrictTypedTypePolicies = {
	AvailableActionsInterfaceType?: Omit<TypePolicy, "fields" | "keyFields"> & {
		keyFields?: false | AvailableActionsInterfaceTypeKeySpecifier | (() => undefined | AvailableActionsInterfaceTypeKeySpecifier),
		fields?: AvailableActionsInterfaceTypeFieldPolicy,
	},
	Category?: Omit<TypePolicy, "fields" | "keyFields"> & {
		keyFields?: false | CategoryKeySpecifier | (() => undefined | CategoryKeySpecifier),
		fields?: CategoryFieldPolicy,
	},
	IProduct?: Omit<TypePolicy, "fields" | "keyFields"> & {
		keyFields?: false | IProductKeySpecifier | (() => undefined | IProductKeySpecifier),
		fields?: IProductFieldPolicy,
	},
	MutationType?: Omit<TypePolicy, "fields" | "keyFields"> & {
		keyFields?: false | MutationTypeKeySpecifier | (() => undefined | MutationTypeKeySpecifier),
		fields?: MutationTypeFieldPolicy,
	},
	PageInfo?: Omit<TypePolicy, "fields" | "keyFields"> & {
		keyFields?: false | PageInfoKeySpecifier | (() => undefined | PageInfoKeySpecifier),
		fields?: PageInfoFieldPolicy,
	},
	Product?: Omit<TypePolicy, "fields" | "keyFields"> & {
		keyFields?: false | ProductKeySpecifier | (() => undefined | ProductKeySpecifier),
		fields?: ProductFieldPolicy,
	},
	ProductConnection?: Omit<TypePolicy, "fields" | "keyFields"> & {
		keyFields?: false | ProductConnectionKeySpecifier | (() => undefined | ProductConnectionKeySpecifier),
		fields?: ProductConnectionFieldPolicy,
	},
	ProductContextType?: Omit<TypePolicy, "fields" | "keyFields"> & {
		keyFields?: false | ProductContextTypeKeySpecifier | (() => undefined | ProductContextTypeKeySpecifier),
		fields?: ProductContextTypeFieldPolicy,
	},
	ProductEdge?: Omit<TypePolicy, "fields" | "keyFields"> & {
		keyFields?: false | ProductEdgeKeySpecifier | (() => undefined | ProductEdgeKeySpecifier),
		fields?: ProductEdgeFieldPolicy,
	},
	ProductInBackorder?: Omit<TypePolicy, "fields" | "keyFields"> & {
		keyFields?: false | ProductInBackorderKeySpecifier | (() => undefined | ProductInBackorderKeySpecifier),
		fields?: ProductInBackorderFieldPolicy,
	},
	ProductInTransit?: Omit<TypePolicy, "fields" | "keyFields"> & {
		keyFields?: false | ProductInTransitKeySpecifier | (() => undefined | ProductInTransitKeySpecifier),
		fields?: ProductInTransitFieldPolicy,
	},
	QueryType?: Omit<TypePolicy, "fields" | "keyFields"> & {
		keyFields?: false | QueryTypeKeySpecifier | (() => undefined | QueryTypeKeySpecifier),
		fields?: QueryTypeFieldPolicy,
	},
	SubscriptionType?: Omit<TypePolicy, "fields" | "keyFields"> & {
		keyFields?: false | SubscriptionTypeKeySpecifier | (() => undefined | SubscriptionTypeKeySpecifier),
		fields?: SubscriptionTypeFieldPolicy,
	}
};
export type TypedTypePolicies = StrictTypedTypePolicies & TypePolicies;