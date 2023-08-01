import React, { useEffect, useState } from "react";

import { HStack, Button } from "@chakra-ui/react";

import { useSearchParams } from "react-router-dom";

import { DiffResultIs } from "./common";

const usePrevious = <T,>(value: T): T | null => {
  // The ref object is a generic container whose current property is mutable ...
  // ... and can hold any value, similar to an instance property on a class
  const ref = React.useRef<T | null>(null);
  // Store current value in ref
  useEffect(() => {
    ref.current = value;
  }, [value]); // Only re-run if value changes
  // Return previous value (happens before update in useEffect above)
  return ref.current;
};

export interface SchemaFiltersStatistics {
  total: number;
  added: number;
  updated: number;
  removed: number;
}
interface HttpSchemaFiltersProps {
  onFiltersChanged: (types: DiffResultIs[]) => void;
  statistics: SchemaFiltersStatistics;
  defaults?: DiffResultIs[];
}

const HttpSchemaFilters: React.FC<HttpSchemaFiltersProps> = ({
  statistics,
  defaults = [],
  onFiltersChanged,
}: HttpSchemaFiltersProps) => {
  const [searchParams, _] = useSearchParams();

  let defaultFilters: DiffResultIs[] = [];

  const diffTypeFilters = searchParams.get("diffTypeFilters");
  if (diffTypeFilters) {
    defaultFilters = diffTypeFilters.split(",").map((element) => {
      switch (element) {
        case "added":
          return "+";
        case "removed":
          return "-";
        case "updated":
          return "~";
        default:
          return "=";
      }
    }) as DiffResultIs[];
  }

  if (defaults && defaults.length > 0) {
    if (statistics.added || statistics.updated || statistics.removed) {
      defaultFilters = defaults;
    }
  }

  const [all, setAll] = useState<boolean>(defaultFilters.length === 0);

  const [added, setAdded] = useState<boolean>(
    defaultFilters.includes(DiffResultIs.Added)
  );
  const [updated, setUpdated] = useState<boolean>(
    defaultFilters.includes(DiffResultIs.Updated)
  );
  const [removed, setRemoved] = useState<boolean>(
    defaultFilters.includes(DiffResultIs.Removed)
  );

  const [selected, setSelected] = useState<DiffResultIs[]>(defaultFilters);

  const lastSelected = usePrevious(selected);

  useEffect(() => {
    const equals =
      lastSelected !== null &&
      selected.length === lastSelected.length &&
      selected.every((v, idx) => v === lastSelected[idx]);

    if (!equals) {
      onFiltersChanged(selected);
    }
  }, [onFiltersChanged, defaultFilters, selected]);

  useEffect(() => {
    if (all) {
      setSelected([]);
      return;
    }
    let types = [];
    if (added) {
      types.push(DiffResultIs.Added);
    }
    if (updated) {
      types.push(DiffResultIs.Updated);
    }
    if (removed) {
      types.push(DiffResultIs.Removed);
    }
    setSelected(types);
  }, [all, added, updated, removed, setSelected]);

  const setAllFilter = (value: boolean) => {
    if (value) {
      setAdded(false);
      setUpdated(false);
      setRemoved(false);
    }

    setAll(value);
  };

  const setAddedFilter = (value: boolean) => {
    if (value) {
      setAll(false);
    }
    setAdded(value);
  };

  const setUpdatedFilter = (value: boolean) => {
    if (value) {
      setAll(false);
    }
    setUpdated(value);
  };

  const setRemovedFilter = (value: boolean) => {
    if (value) {
      setAll(false);
    }
    setRemoved(value);
  };

  return (
    <HStack m={2}>
      <Button
        variant={all ? "solid" : "ghost"}
        onClick={() => setAllFilter(!all)}
      >
        All ({statistics.total})
      </Button>
      <Button
        variant={added ? "solid" : "ghost"}
        onClick={() => setAddedFilter(!added)}
        colorScheme="green"
      >
        Added ({statistics.added})
      </Button>
      <Button
        variant={updated ? "solid" : "ghost"}
        onClick={() => setUpdatedFilter(!updated)}
        colorScheme="orange"
      >
        Updated ({statistics.updated})
      </Button>
      <Button
        variant={removed ? "solid" : "ghost"}
        onClick={() => setRemovedFilter(!removed)}
        colorScheme="red"
      >
        Removed ({statistics.removed})
      </Button>
    </HStack>
  );
};

export default HttpSchemaFilters;
