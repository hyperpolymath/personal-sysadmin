;; SPDX-License-Identifier: AGPL-3.0-or-later
;; ECOSYSTEM.scm - Project relationships and ecosystem position

(ecosystem
  ((version . "1.0.0")
   (name . "personal-sysadmin")
   (type . "application")
   (purpose . "AI-assisted Linux system administration toolkit")

   (position-in-ecosystem
     "Personal Sysadmin (PSA) is the diagnostic and management layer for
      individual Linux machines. It provides Sysinternals-like capabilities
      enhanced with AI reasoning, learning from solutions, and mesh-based
      knowledge sharing across devices.")

   (related-projects
     ((system-emergency-room
        ((relationship . sibling-standard)
         (description . "Emergency triage and crisis management for systems")
         (integration . "PSA can escalate critical issues to Emergency Room")
         (shared-protocols . ("crisis-escalation" "health-metrics"))))

      (system-operating-theatre
        ((relationship . sibling-standard)
         (description . "Surgical intervention for complex system repairs")
         (integration . "PSA provides pre-operative diagnostics")
         (shared-protocols . ("system-state" "repair-plan"))))

      (feedback-o-tron
        ((relationship . sibling-standard)
         (description . "Feedback loop management across the system suite")
         (integration . "Receives learning signals from PSA solutions")
         (shared-protocols . ("outcome-tracking" "confidence-signals"))))

      (ambientops
        ((relationship . potential-consumer)
         (description . "Ambient operations and automation platform")
         (integration . "Could consume PSA health metrics and diagnostics")))

      (hybrid-automation-router
        ((relationship . potential-consumer)
         (description . "Routes automation between human and AI agents")
         (integration . "PSA rules could feed HAR routing decisions")))

      (gitvisor
        ((relationship . inspiration)
         (description . "Git repository management and monitoring")
         (integration . "PSA rules store uses git-like versioning inspired by gitvisor")))))

   (what-this-is
     ("AI-assisted sysadmin toolkit for Linux"
      "Learning system that crystallizes proven solutions into rules"
      "Mesh-based knowledge sharing across personal devices"
      "Sysinternals-like process, network, disk, security tools"
      "Forum scraper and solution compiler"
      "miniKanren-inspired reasoning engine"))

   (what-this-is-not
     ("Not a replacement for system-emergency-room (crisis handling)"
      "Not a monitoring/alerting system (that's observability's job)"
      "Not a configuration management tool (that's SaltStack/Ansible)"
      "Not cloud-focused (personal/edge device oriented)"
      "Not a container orchestrator"))))
