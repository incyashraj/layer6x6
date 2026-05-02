#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd -- "${SCRIPT_DIR}/.." && pwd)"
BASELINE="${REPO_ROOT}/docs/book/src/phase1/benchmark-baseline.json"
CRITERION_DIR="${REPO_ROOT}/target/criterion"

ruby -rjson -e '
baseline = JSON.parse(File.read(ARGV.fetch(0)))
criterion_dir = ARGV.fetch(1)
warned = false

baseline.fetch("metrics").each do |metric, spec|
  file = File.join(criterion_dir, spec.fetch("criterion_path"), "new", "estimates.json")
  unless File.exist?(file)
    puts "::warning::missing Criterion estimate for #{metric}: #{file}"
    next
  end

  estimates = JSON.parse(File.read(file))
  current = estimates.fetch("mean").fetch("point_estimate")
  baseline_ns = spec.fetch("baseline_ns")
  allowed = baseline_ns * 1.10

  if current > allowed
    warned = true
    pct = ((current - baseline_ns) / baseline_ns.to_f * 100).round(1)
    puts "::warning::#{metric} regressed by #{pct}% (current #{current.round} ns, baseline #{baseline_ns.round} ns)"
  else
    puts "#{metric}: #{current.round} ns (baseline #{baseline_ns.round} ns)"
  end
end

exit 0
' "${BASELINE}" "${CRITERION_DIR}"
