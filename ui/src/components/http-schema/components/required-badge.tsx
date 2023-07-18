import { Text } from "@chakra-ui/react";
import {
  DiffResult,
  getBgColor,
  selectValue,
  PrimaryOperation,
} from "../common";

interface RequiredBadgeProps {
  required: DiffResult<boolean | string> | undefined;
  primary: PrimaryOperation;
}

export const RequiredBadge: React.FC<RequiredBadgeProps> = ({
  primary,
  required,
}) => {
  if (!required) {
    return null;
  }

  const requiredComponent = selectValue(
    required,
    primary,
    (value) => (value ? "*" : ""),
    (value) => (value ? "*" : ""),
    // show if updated or deleted in any case
    (value) => "*",
    (value) => "*"
  );

  let requiredBgColor = getBgColor(required, primary);
  return (
    <Text as="span" color="red" display="inline" bgColor={requiredBgColor}>
      {requiredComponent}
    </Text>
  );
};

export default RequiredBadge;
