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
  VStack,
} from "@chakra-ui/react";

import { useForm } from "react-hook-form";
import { useNavigate } from "react-router-dom";

import { Version } from "../versions/models";
import { Project } from "../projects/models";
import api from "../../api";

interface VersionCompareModalProps {
  project: Project;
  defaultSourceBranch: string;
  isOpen: boolean;
  onClose: () => void;
}

export const VersionCompareModal: React.FC<VersionCompareModalProps> = ({
  project,
  defaultSourceBranch,
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
      sourceBranch: defaultSourceBranch,
      targetId: 0,
      targetBranch: project.branches[0],
    },
  });

  const navigate = useNavigate();

  const [sourceVersions, setSourceVersions] = useState<Version[]>([]);
  const [targetVersions, setTargetVersions] = useState<Version[]>([]);

  let sourceBranch = watch("sourceBranch");
  useEffect(() => {
    api.versions.list(project.slug, sourceBranch).then(({ data }) => {
      setSourceVersions(data);
      if (data.length > 0) {
        setValue("sourceId", data[0].id);
      }
    });
  }, [project, sourceBranch, setSourceVersions, setValue, watch]);

  let targetBranch = watch("targetBranch");
  useEffect(() => {
    api.versions.list(project.slug, targetBranch).then(({ data }) => {
      setTargetVersions(data);
      if (data.length > 0) {
        setValue("targetId", data[0].id);
      }
    });
  }, [project, targetBranch, setTargetVersions, setValue, watch]);

  const onSubmit = (values: any) => {
    navigate(
      `../${values.sourceBranch}/${values.sourceId}/compare/${values.targetBranch}/${values.targetId}`,
      { relative: "path" }
    );
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
          <VStack alignItems="stretch">
            <HStack spacing={3} justifyContent="stretch">
              <Stack spacing={3} flex={1}>
                <FormControl isInvalid={!!errors.sourceBranch}>
                  <FormLabel htmlFor="sourceBranch">Source branch</FormLabel>
                  <Select {...register("sourceBranch")}>
                    {project.branches.map((branch) => (
                      <option key={branch} value={branch}>
                        {branch}
                      </option>
                    ))}
                  </Select>
                  {!!errors.sourceBranch && (
                    <FormErrorMessage>
                      {errors.sourceBranch.message}
                    </FormErrorMessage>
                  )}
                </FormControl>
              </Stack>

              <Stack spacing={3} flex={1}>
                <FormControl isInvalid={!!errors.targetBranch}>
                  <FormLabel htmlFor="targetBranch">Target branch</FormLabel>
                  <Select {...register("targetBranch")}>
                    {project.branches.map((branch) => (
                      <option key={branch} value={branch}>
                        {branch}
                      </option>
                    ))}
                  </Select>
                  {errors.targetBranch && (
                    <FormErrorMessage>
                      {errors.targetBranch.message}
                    </FormErrorMessage>
                  )}
                </FormControl>
              </Stack>
            </HStack>
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
                    {sourceVersions.map((version) => (
                      <option key={version.id} value={version.id}>
                        [{version.id}] {version.message}
                      </option>
                    ))}
                  </Select>
                  {!!errors.sourceId && (
                    <FormErrorMessage>
                      {errors.sourceId.message}
                    </FormErrorMessage>
                  )}
                </FormControl>
              </Stack>

              <Stack spacing={3} flex={1}>
                <FormControl isInvalid={!!errors.targetId}>
                  <FormLabel htmlFor="targetId">Target version</FormLabel>
                  <Select {...register("targetId")}>
                    {targetVersions.map((version) => (
                      <option key={version.id} value={version.id}>
                        [{version.id}] {version.message}
                      </option>
                    ))}
                  </Select>
                  {errors.targetId && (
                    <FormErrorMessage>
                      {errors.targetId.message}
                    </FormErrorMessage>
                  )}
                </FormControl>
              </Stack>
            </HStack>
          </VStack>
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
