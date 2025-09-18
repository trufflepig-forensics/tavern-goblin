import { createFileRoute } from '@tanstack/react-router'
import Profile from "src/components/profile";

export const Route = createFileRoute('/_menu/profile/')({
  component: Profile,
})