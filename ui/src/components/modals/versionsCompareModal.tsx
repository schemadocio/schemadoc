import React, { useEffect, useState } from "react";
import {
  Stack,
  HStack,
  Button,
  Modal,
  ModalOverlay,
  ModalContent,
  ModalHeader,
  ModalCloseButton,
  ModalBody,
  ModalFooter,
  FormControl,
  FormLabel,
  FormErrorMessage,
  Select,
} from "@chakra-ui/react";

import { useForm } from "react-hook-form";
import { useNavigate } from "react-router-dom";

import { Version } from "../versions/models";
import { Project } from "../projects/models";
import api from "../../api";

interface VersionCompareModalProps {
  project: Project;
  isOpen: boolean;
  onClose: () => void;
}

export const VersionCompareModal: React.FC<VersionCompareModalProps> = ({
  project,
  isOpen,
  onClose,
}) => {
  const {
    watch,
    register,
    setValue,
    handleSubmit,
    formState: { errors },
  } = useForm({
    defaultValues: {
      sourceId: 0,
      targetId: 0,
    },
  });

  const navigate = useNavigate();

  const [versions, setVersions] = useState<Version[]>([]);

  useEffect(() => {
    api.versions.list(project.slug).then(({ data }) => {
      setVersions(data);
      if (data.length > 0) {
        setValue("sourceId", data[0].id);
        setValue("targetId", data[0].id);
      }
    });
  }, [project, setVersions, setValue]);

  const onSubmit = (values: any) => {
    navigate(`${values.sourceId}/compare/${values.targetId}`);
  };

  const { ref: sourceIdRef, ...sourceIdRest } = register("sourceId");
  const focusRef = React.useRef(null);

  return (
    <Modal
      size="xl"
      isOpen={isOpen}
      onClose={onClose}
      initialFocusRef={focusRef}
    >
      <ModalOverlay />
      <ModalContent>
        <ModalHeader>Compare API versions</ModalHeader>
        <ModalCloseButton />
        <ModalBody>
          <HStack spacing={3} justifyContent="stretch">
            <Stack spacing={3} flex={1}>
              <FormControl isInvalid={!!errors.sourceId}>
                <FormLabel htmlFor="sourceId">Source version</FormLabel>
                <Select
                  {...sourceIdRest}
                  ref={(e: any) => {
                    sourceIdRef(e);
                    focusRef.current = e;
                  }}
                >
                  {versions.map((version) => (
                    <option key={version.id} value={version.id}>
                      [{version.id}] {version.message}
                    </option>
                  ))}
                </Select>
                {!!errors.sourceId && (
                  <FormErrorMessage>{errors.sourceId.message}</FormErrorMessage>
                )}
              </FormControl>
            </Stack>

            <Stack spacing={3} flex={1}>
              <FormControl isInvalid={!!errors.targetId}>
                <FormLabel htmlFor="targetId">Target version</FormLabel>
                <Select {...register("targetId")}>
                  {versions.map((version) => (
                    <option key={version.id} value={version.id}>
                      [{version.id}] {version.message}
                    </option>
                  ))}
                </Select>
                {errors.targetId && (
                  <FormErrorMessage>{errors.targetId.message}</FormErrorMessage>
                )}
              </FormControl>
            </Stack>
          </HStack>
        </ModalBody>

        <ModalFooter>
          <Button
            onClick={handleSubmit(onSubmit)}
            isDisabled={watch("sourceId") === watch("targetId")}
          >
            Compare
          </Button>
        </ModalFooter>
      </ModalContent>
    </Modal>
  );
};

export default VersionCompareModal;
