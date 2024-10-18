import { createPersist } from "effector-storage";
import { createDomain, Domain, Store } from "effector";

const persist = createPersist({ storeName: "context", timeout: 200 });

const storePersistence = (store: Store<unknown>) =>
	persist({ store, dbName: store.compositeName.path.at(0) });

const withPersistence = (domain: Domain) => {
	void domain.onCreateStore(storePersistence);

	return domain;
};

export const createRoot = (name: string) => withPersistence(createDomain(name));

export type AbstractUnit = {
	[key: string]: unknown;
	domain: Domain;
};

export type SingletonService = AbstractUnit;
